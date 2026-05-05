use std::vec;

use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::exit::{Exit, ExitKind, ExitPlacement, ExitWall, ExitWidth};

pub fn place_door(shape: &mut Grid, exit_placement: ExitPlacement) {
    let grid_width = shape.columns;
    let start = grid_index(exit_placement.row, exit_placement.column, grid_width);
    let door = Node::Tile(Tile {
        kind: TileKind::Door,
        icon: TileIcon::Door
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    for h in 0..exit_placement.height {
        for w in 0..exit_placement.width {
            let node = if h == 0 && w == 0 { door.clone() } else { floor.clone() };
            shape.nodes[start + w + (h * grid_width)] = node;
        }
    }
}

pub fn place_passage(shape: &mut Grid, exit_placement: ExitPlacement) {
    let grid_width = shape.columns;
    let start = grid_index(exit_placement.row, exit_placement.column, grid_width);
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    for h in 0..exit_placement.height {
        for w in 0..exit_placement.width {
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

pub fn place_exits(shape: &mut Grid, exits: &[Exit]) {
    for exit in exits.iter() {
         match exit.kind {
             ExitKind::Door => place_door(shape, calculate_exit_placement(shape, exit.wall, exit.width)),
             ExitKind::Passage => place_passage(shape, calculate_exit_placement(exit.wall, exit.width)),
             _ => panic!("Unsupported exit kind.")
        }
    }
}

pub fn calculate_exit_placement(shape_size: usize, shape_columns: usize, exit: &Exit) -> ExitPlacement {
    let shape_rows = (shape_size / shape_columns) as u8;
    const NORTH_ROW: usize = 0;
    const NORTH_HEIGHT: usize = 2;
    let south_row: usize = shape_rows - 2 as usize;
    const SOUTH_HEIGHT: usize = 2;
    let east_column: usize = shape_columns - 2;
    const EAST_WIDTH: usize = 2;
    const WEST_COLUMN: usize = 0;
    const WEST_WIDTH: usize = 2;
    match (exit.wall, exit.width) {
        (ExitWall::North, ExitWidth::Five) => {
            ExitPlacement::new(
                NORTH_ROW, 
                (dice::roll(shape_rows - 4 as u8) + 1) as usize, // 2-5
                1,
                NORTH_HEIGHT
            )
        },
        (ExitWall::North, ExitWidth::Ten) => {
            ExitPlacement::new(
                NORTH_ROW,
                (dice::roll(3) + 1) as usize, // 2-4,
                2,
                NORTH_HEIGHT
            )
        },
        (ExitWall::North, ExitWidth::Twenty) => {
            ExitPlacement::new(
                NORTH_ROW,
                2,
                4,
                NORTH_HEIGHT
            )
        },
        (ExitWall::South, ExitWidth::Five) => {
            ExitPlacement::new(
                south_row,
                (dice::roll(4) + 1) as usize, // 2-5
                1,
                SOUTH_HEIGHT
            )
        },
        (ExitWall::South, ExitWidth::Ten) => {
            ExitPlacement::new(
                south_row,
                (dice::roll(3) + 1) as usize, // 2-4
                2,
                SOUTH_HEIGHT
            )
        },
        (ExitWall::South, ExitWidth::Twenty) => {
            ExitPlacement::new(
                south_row,
                2,
                4,
                SOUTH_HEIGHT
            )
        },
        (ExitWall::East, ExitWidth::Five) => {
            ExitPlacement::new(
                (dice::roll(4) + 1) as usize, // 2-5
                east_column,
                EAST_WIDTH,
                1
            )
        },
        (ExitWall::East, ExitWidth::Ten) => {
            ExitPlacement::new(
                (dice::roll(3) + 1) as usize, // 2-4
                east_column,
                EAST_WIDTH,
                2)
        },
        (ExitWall::East, ExitWidth::Twenty) => {
            ExitPlacement::new(
                2,
                east_column,
                EAST_WIDTH,
                4
            )
        },
        (ExitWall::West, ExitWidth::Five) => {
            ExitPlacement::new(
                (dice::roll(4) + 1) as usize, // 2-5
                WEST_COLUMN,
                WEST_WIDTH,
                1
            )
        },
        (ExitWall::West, ExitWidth::Ten) => {
            ExitPlacement::new(
                (dice::roll(3) + 1) as usize, // 2-4
                WEST_COLUMN,
                WEST_WIDTH,
                2
            )
        },
        (ExitWall::West, ExitWidth::Twenty) => {
            ExitPlacement::new(
                2,
                WEST_COLUMN,
                WEST_WIDTH,
                4
            )
        }
    }
}

pub fn shuffle_walls(walls: &mut [ExitWall; 4]) {
    for i in (1..walls.len()).rev() {
        let j = (dice::roll((i + 1) as u8) as usize) - 1;
        walls.swap(i, j);
    }
}
