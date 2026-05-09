use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, ExitLength,  Wall, get_exit_passage, get_random_exit_length, shuffle_walls, get_exit_door};

/*
    Starting Area 4 is a 80x20ft rectangular room with a row of pillars down the middle
    that serves as the starting point for a map.
    There are four guaranteed exits; two doors, one on each short wall, and two passages, one on each long wall.
    Doors may be 5ft wide.
    Passages may be 5/10/20/30/40ft wide.

    # # # # # # # # # # # # # # # # # # # #
    # # # # # # # # # # # # # # # # # # # #
    # #                                 # #
    # #   # #   # #   # #   # #   # #   # #
    # #   # #   # #   # #   # #   # #   # #
    # #                                 # #
    # # # # # # # # # # # # # # # # # # # #
    # # # # # # # # # # # # # # # # # # # #

    or

    # # # # # # # #
    # # # # # # # #
    # #         # #
    # #   # #   # #
    # #   # #   # #
    # #         # #
    # #   # #   # #
    # #   # #   # #
    # #         # #
    # #   # #   # #
    # #   # #   # #
    # #         # #
    # #   # #   # #
    # #   # #   # #
    # #         # #
    # #   # #   # #
    # #   # #   # #
    # #         # #
    # # # # # # # #
    # # # # # # # #
*/
pub fn new() -> Room {
    // Set room properties.
    let num_rows: usize = match dice::roll(2) {
        1 => 8,
        2 => 20,
        _ => panic!("Unexpected dice roll.")
    };
    let num_cols: usize = match num_rows {
        8 => 20,
        20 => 8,
        _ => panic!("Unexpected number of rows.")
    };
    
    const MAX_EXIT_LENGTH: ExitLength = ExitLength::Forty;
    const WALL_THICKNESS: usize = 2;

    const NORTH_WALL_ROW: usize = 0;
    let south_wall_row: usize = num_rows - WALL_THICKNESS;
    let east_wall_col: usize = num_cols - WALL_THICKNESS;
    const WEST_WALL_COL: usize = 0;

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
    // Note: Of the two configurations for this room,
    // this one is easier to build the base shape for,
    // so we'll build this one and rotate it if necessary.
    let mut room_grid = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
    ]);

    if num_cols != 8 {
        // We know we need to be in the horizontal configuration
        room_grid = room_grid.rotate_clockwise();

        // North and South are wide. Need passages there.

        // Determine exit lengths given size of room
        let north_exit_width = get_random_exit_length(MAX_EXIT_LENGTH);
        let south_exit_width = get_random_exit_length(MAX_EXIT_LENGTH);

        // Determine exit placement given exit lengths and valid ranges
        let north_exit_col_index = match north_exit_width {
            ExitLength::Five => dice::roll_range(2, 17) as usize,
            ExitLength::Ten => dice::roll_range(2, 16) as usize,
            ExitLength::Twenty => dice::roll_range(2, 14) as usize,
            ExitLength::Thirty => dice::roll_range(2, 12) as usize,
            ExitLength::Forty => dice::roll_range(2, 10) as usize
        };
        let south_exit_col_index = match south_exit_width {
            ExitLength::Five => dice::roll_range(2, 17) as usize,
            ExitLength::Ten => dice::roll_range(2, 16) as usize,
            ExitLength::Twenty => dice::roll_range(2, 14) as usize,
            ExitLength::Thirty => dice::roll_range(2, 12) as usize,
            ExitLength::Forty => dice::roll_range(2, 10) as usize
        };

        // Generate exit grids given exit lengths and wall orientations.
        let north_exit = get_exit_passage(Wall::North, north_exit_width);
        let south_exit = get_exit_passage(Wall::South, south_exit_width);

        // Update room grid with exits
        room_grid.update(&north_exit, NORTH_WALL_ROW, north_exit_col_index);
        room_grid.update(&south_exit, south_wall_row, south_exit_col_index);

        // East and West are narrow. Need doors there.
        let east_door_index = dice::roll_range(2, 5) as usize;
        let east_door_exit = get_exit_door(Wall::East);
        room_grid.update(&east_door_exit, east_door_index, east_wall_col);

        let west_door_index = dice::roll_range(2, 5) as usize;
        let west_door_exit = get_exit_door(Wall::West);
        room_grid.update(&west_door_exit, west_door_index, WEST_WALL_COL);

        Room::new(room_grid)
    }
    else {
        // We know we need to be in the vertical configuration
        // North and South are narrow. Need doors there.
        let north_door_index = dice::roll_range(2, 5) as usize;
        let north_door_exit = get_exit_door(Wall::North);
        room_grid.update(&north_door_exit, NORTH_WALL_ROW, north_door_index);

        let south_door_index = dice::roll_range(2, 5) as usize;
        let south_door_exit = get_exit_door(Wall::South);
        room_grid.update(&south_door_exit, south_wall_row, south_door_index);

        // East and West are wide. Need passages there.

        // Determine exit lengths given size of room
        let east_exit_width = get_random_exit_length(MAX_EXIT_LENGTH);
        let west_exit_width = get_random_exit_length(MAX_EXIT_LENGTH);

        // Determine exit placement given exit lengths and valid ranges
        let east_exit_row_index = match east_exit_width {
            ExitLength::Five => dice::roll_range(2, 17) as usize,
            ExitLength::Ten => dice::roll_range(2, 16) as usize,
            ExitLength::Twenty => dice::roll_range(2, 14) as usize,
            ExitLength::Thirty => dice::roll_range(2, 12) as usize,
            ExitLength::Forty => dice::roll_range(2, 10) as usize
        };
        let west_exit_row_index = match west_exit_width {
            ExitLength::Five => dice::roll_range(2, 17) as usize,
            ExitLength::Ten => dice::roll_range(2, 16) as usize,
            ExitLength::Twenty => dice::roll_range(2, 14) as usize,
            ExitLength::Thirty => dice::roll_range(2, 12) as usize,
            ExitLength::Forty => dice::roll_range(2, 10) as usize
        };

        // Generate exit grids given exit lengths and wall orientations.
        let east_exit = get_exit_passage(Wall::East, east_exit_width);
        let west_exit = get_exit_passage(Wall::West, west_exit_width);

        // Update room grid with exits
        room_grid.update(&east_exit, east_exit_row_index, east_wall_col);
        room_grid.update(&west_exit, west_exit_row_index, WEST_WALL_COL);

        Room::new(room_grid)
    }
}