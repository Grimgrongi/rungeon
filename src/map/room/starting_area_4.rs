use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, ExitLength,  Wall, get_exit_passage, get_random_exit_length, shuffle_walls, get_exit_door};

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-4
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
    
    const WALL_THICKNESS: usize = 2;

    const NORTH_WALL_ROW: usize = 0;
    let south_wall_row: usize = num_rows - WALL_THICKNESS;
    let east_wall_col: usize = num_cols - WALL_THICKNESS;
    const WEST_WALL_COL: usize = 0;

    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    if dice::roll(2) == 1 {
        // Let's build a horizontal starting area 4
        let mut starting_area4 = Grid::new(20, vec![
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
        ]);

        starting_area4 = place_door(starting_area4, Wall::East);
        starting_area4 = place_door(starting_area4, Wall::West);
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::North, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::North, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::South, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::South, 10);
        }
        Room::new(starting_area4, exits)
    }
    else {
        // Let's build a vertical starting area 4
        let mut starting_area4 = Grid::new(8, vec![
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),wall.clone(), wall.clone(), floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
            wall.clone(),wall.clone(),wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
        ]);

        starting_area4 = place_door(starting_area4, Wall::North);
        starting_area4 = place_door(starting_area4, Wall::South);
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::East, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::East, 10);
        }
        if dice::roll(12) <= 2 { // This will be a 5ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::West, 5);
        }
        else { // This will be a 10ft wide passage
            starting_area4 = place_passage(starting_area4, Wall::West, 10);
        }
        Room::new(starting_area4, exits)
    }
}