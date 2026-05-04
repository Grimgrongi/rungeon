use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::exit::{ExitWall, ExitWidth};

pub struct Placement {
    pub row: usize,
    pub column: usize,
    pub width: usize,
    pub height: usize
}

impl Placement {
    pub fn new(row: usize, column: usize, width: usize, height: usize) -> Placement {
        Placement {
            row,
            column,
            width,
            height
        }
    }
}

pub fn add_door(shape: &mut Grid, placement: Placement) {
    let grid_width = shape.columns;
    let start = grid_index(placement.row, placement.column, grid_width);
    let door = Node::Tile(Tile {
        kind: TileKind::Door,
        icon: TileIcon::Door
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    for h in 0..placement.height {
        for w in 0..placement.width {
            let node = if h == 0 && w == 0 { door.clone() } else { floor.clone() };
            shape.nodes[start + w + (h * grid_width)] = node;
        }
    }
}

pub fn add_passage(shape: &mut Grid, placement: Placement) {
    let grid_width = shape.columns;
    let start = grid_index(placement.row, placement.column, grid_width);
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    for h in 0..placement.height {
        for w in 0..placement.width {
            shape.nodes[start + w + (h * grid_width)] = floor.clone();
        }
    }
}

pub fn get_random_exit_width(max_width: ExitWidth) -> ExitWidth {
    match max_width {
        ExitWidth::Five => ExitWidth::Five,
        ExitWidth::Ten => {
            match dice::roll(12) {
                1..=2 => ExitWidth::Five,
                3..=12 => ExitWidth::Ten,
                _ => panic!("Unexpected dice roll.")
            }
        },
        ExitWidth::Twenty => {
            match dice::roll(14) {
                1..=2 => ExitWidth::Five,
                3..=12 => ExitWidth::Ten,
                13..=14 => ExitWidth::Twenty,
                _ => panic!("Unexpected dice roll.")
            }
        }
    }
}

pub fn grid_index(row: usize, column: usize, grid_width: usize) -> usize {
    row * grid_width + column
}

pub fn placement_on_wall(wall: ExitWall, width: ExitWidth) -> Placement {
    const NORTH_ROW: usize = 0;
    const NORTH_HEIGHT: usize = 2;
    const SOUTH_ROW: usize = 6;
    const SOUTH_HEIGHT: usize = 2;
    const EAST_COL: usize = 6;
    const EAST_WIDTH: usize = 2;
    const WEST_COL: usize = 0;
    const WEST_WIDTH: usize = 2;
    match (wall, width) {
        (ExitWall::North, ExitWidth::Five) => {
            let column = (dice::roll(4) + 1) as usize; // 2-5
            Placement::new(NORTH_ROW, column, 1, NORTH_HEIGHT)
        },
        (ExitWall::North, ExitWidth::Ten) => {
            let column = (dice::roll(3) + 1) as usize; // 2-4
            Placement::new(NORTH_ROW, column, 2, NORTH_HEIGHT)
        },
        (ExitWall::North, ExitWidth::Twenty) => {
            Placement::new(NORTH_ROW, 2, 4, NORTH_HEIGHT)
        },
        (ExitWall::South, ExitWidth::Five) => {
            let column = (dice::roll(4) + 1) as usize; // 2-5
            Placement::new(SOUTH_ROW, column, 1, SOUTH_HEIGHT)
        },
        (ExitWall::South, ExitWidth::Ten) => {
            let column = (dice::roll(3) + 1) as usize; // 2-4
            Placement::new(SOUTH_ROW, column, 2, SOUTH_HEIGHT)
        },
        (ExitWall::South, ExitWidth::Twenty) => {
            Placement::new(SOUTH_ROW, 2, 4, SOUTH_HEIGHT)
        },
        (ExitWall::East, ExitWidth::Five) => {
            let row = (dice::roll(4) + 1) as usize; // 2-5
            Placement::new(row, EAST_COL, EAST_WIDTH, 1)
        },
        (ExitWall::East, ExitWidth::Ten) => {
            let row = (dice::roll(3) + 1) as usize; // 2-4
            Placement::new(row, EAST_COL, EAST_WIDTH, 2)
        },
        (ExitWall::East, ExitWidth::Twenty) => {
            Placement::new(2, EAST_COL, EAST_WIDTH, 4)
        },
        (ExitWall::West, ExitWidth::Five) => {
            let row = (dice::roll(4) + 1) as usize; // 2-5
            Placement::new(row, WEST_COL, WEST_WIDTH, 1)
        },
        (ExitWall::West, ExitWidth::Ten) => {
            let row = (dice::roll(3) + 1) as usize; // 2-4
            Placement::new(row, WEST_COL, WEST_WIDTH, 2)
        },
        (ExitWall::West, ExitWidth::Twenty) => {
            Placement::new(2, WEST_COL, WEST_WIDTH, 4)
        }
    }
}

pub fn shuffle_walls(walls: &mut [ExitWall; 4]) {
    for i in (1..walls.len()).rev() {
        let j = (dice::roll((i + 1) as u8) as usize) - 1;
        walls.swap(i, j);
    }
}
