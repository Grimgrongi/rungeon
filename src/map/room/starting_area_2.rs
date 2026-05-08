use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, ExitLength,  Wall, get_exit_passage, get_random_exit_length, shuffle_walls, get_exit_door};

/*
    Starting Area 2 is a 20x20ft square room that serves as the starting point for a map.
    There are three guaranteed exits; two doors and one passage.
    Doors may be 5ft wide.
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
    let door = Node::Tile(Tile {
        kind: TileKind::Door,
        icon: TileIcon::Door
    });

    // Initialize the base shape of the room.
    let mut room_grid = Grid::new(NUM_COLS, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
    ]);

    // Randomly select walls for the exits.
    let mut walls = [Wall::North, Wall::South, Wall::East, Wall::West];
    shuffle_walls(&mut walls);

    for (index, wall) in walls.iter().take(3).enumerate() {
        match index {
            // Handle the Passage first
            1 => {
                let passage_length = get_random_exit_length(MAX_EXIT_LENGTH);
                let passage_index = match passage_length {
                    ExitLength::Five => dice::roll_range(2, 5) as usize,
                    ExitLength::Ten => dice::roll_range(2, 4) as usize,
                    ExitLength::Twenty => 2 as usize,
                    _ => panic!("Invalid exit length for starting_area_2: {:?}", passage_length)
                };
                match wall {
                    Wall::North => {
                        let north_exit = get_exit_passage(Wall::North, passage_length);
                        room_grid.update(&north_exit, NORTH_WALL_ROW, passage_index);
                    },
                    Wall::South => {
                        let south_exit = get_exit_passage(Wall::South, passage_length);
                        room_grid.update(&south_exit, SOUTH_WALL_ROW, passage_index);
                    },
                    Wall::East => {
                        let east_exit = get_exit_passage(Wall::East, passage_length);
                        room_grid.update(&east_exit, passage_index, EAST_WALL_COL);
                    },
                    Wall::West => {
                        let west_exit = get_exit_passage(Wall::West, passage_length);
                        room_grid.update(&west_exit, passage_index, WEST_WALL_COL);
                    }
                }
            },
            _ => {
                let door_index = dice::roll_range(2, 5) as usize;
                let door_exit = get_exit_door(*wall);
                match *wall {
                    Wall::North => room_grid.update(&door_exit, NORTH_WALL_ROW, door_index),
                    Wall::South => room_grid.update(&door_exit, SOUTH_WALL_ROW, door_index),
                    Wall::East => room_grid.update(&door_exit, door_index, EAST_WALL_COL),
                    Wall::West => room_grid.update(&door_exit, door_index, WEST_WALL_COL)
                }
            }
        }
    }

    Room::new(room_grid)
}
