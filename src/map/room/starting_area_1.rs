use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, ExitLength, get_random_exit_length};
// use crate::map::room::exit::{Exit, ExitKind, ExitPlacement};

/*
    Starting Area 1 is a 20x20ft square room that serves as the starting point for a map.
    There are four guaranteed exits; one passage on each wall.
    Passages may be 5/10/20ft wide.

    # # # # # # # #
    # # # # # # # #
    # #         # #
    # #         # #
    # #         # #
    # #         # #
    # # # # # # # #
    # # # # # # # #
*/

pub fn new() -> Room {
    // Set room properties.
    const NUM_ROWS: usize = 8;
    const NUM_COLS: usize = 8;
    const WALL_THICKNESS: usize = 2;

    const NORTH_WALL_ROW: usize = 0;
    const SOUTH_WALL_ROW: usize = NUM_ROWS - WALL_THICKNESS;
    const EAST_WALL_COL: usize = NUM_COLS - WALL_THICKNESS;
    const WEST_WALL_COL: usize = 0;
    
    const MAX_EXIT_LENGTH: ExitLength = ExitLength::Twenty;

    // Initialize building blocks for the room.
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let flor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    // Initialize the base shape of the room.
    let mut room_grid = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
    ]);

    // Determine exit lengths given size of room
    let north_exit_width = get_random_exit_length(MAX_EXIT_LENGTH);
    let south_exit_width = get_random_exit_length(MAX_EXIT_LENGTH);
    let east_exit_height = get_random_exit_length(MAX_EXIT_LENGTH);
    let west_exit_height = get_random_exit_length(MAX_EXIT_LENGTH);

    // Determine exit placement given exit lengths and valid ranges
    let north_exit_row_index = match north_exit_width {
        ExitLength::Five => dice::roll_range(2, 5) as usize,
        ExitLength::Ten => dice::roll_range(2, 4) as usize,
        ExitLength::Twenty => 2 as usize
    };
    let south_exit_row_index = match south_exit_width {
        ExitLength::Five => dice::roll_range(2, 5) as usize,
        ExitLength::Ten => dice::roll_range(2, 4) as usize,
        ExitLength::Twenty => 2 as usize
    };
    let east_exit_row_index = match east_exit_height {
        ExitLength::Five => dice::roll_range(2, 5) as usize,
        ExitLength::Ten => dice::roll_range(2, 4) as usize,
        ExitLength::Twenty => 2 as usize
    };
    let west_exit_row_index = match west_exit_height {
        ExitLength::Five => dice::roll_range(2, 5) as usize,
        ExitLength::Ten => dice::roll_range(2, 4) as usize,
        ExitLength::Twenty => 2 as usize
    };

    // Generate exit grids given exit lengths and wall orientations.
    let north_exit = get_exit_passage(Wall::North, north_exit_width);
    let south_exit = get_exit_passage(Wall::South, south_exit_width);
    let east_exit = get_exit_passage(Wall::East, east_exit_height);
    let west_exit = get_exit_passage(Wall::West, west_exit_height);

    // Update room grid with exits
    room_grid.update(&north_exit, NORTH_WALL_ROW, north_exit_row_index);
    room_grid.update(&south_exit, SOUTH_WALL_ROW, south_exit_row_index);
    room_grid.update(&east_exit, east_exit_row_index, EAST_WALL_COL);
    room_grid.update(&west_exit, west_exit_row_index, WEST_WALL_COL);

    // Return the completed room.
    Room::new(room_grid)
}

pub fn get_exit_passage(wall: Wall, width: ExitLength) -> Grid {
    let flor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    let num_columns = if wall == Wall::East || wall == Wall::West {
        2
    }
    else {
        match width {
            ExitLength::Five => 1,
            ExitLength::Ten => 2,
            ExitLength::Twenty => 4,
        }
    };

    Grid::new(
        num_columns,
        match width {
            ExitLength::Five => vec![
                flor.clone(), flor.clone()
            ],
            ExitLength::Ten => vec![
                flor.clone(), flor.clone(),
                flor.clone(), flor.clone()
            ],
            ExitLength::Twenty => vec![
                flor.clone(), flor.clone(), flor.clone(), flor.clone(),
                flor.clone(), flor.clone(), flor.clone(), flor.clone()
            ]
        }
    )
}

#[derive(Copy, Clone, PartialEq)]
pub enum Wall {
    North,
    South,
    East,
    West
}