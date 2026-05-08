use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, Wall, shuffle_walls, get_exit_door};

/*
    Starting Area 3 is a 40x40ft square room that serves as the starting point for a map.
    There are three guaranteed exits; three doors.
    Doors may be 5ft wide.

    # # # # # # # # # # # #
    # # # # # # # # # # # #
    # #                 # #
    # #                 # #
    # #                 # #
    # #                 # #
    # #                 # #
    # #                 # #
    # #                 # #
    # #                 # #
    # # # # # # # # # # # #
    # # # # # # # # # # # #
*/
pub fn new() -> Room {
    // Set room properties.
    const NUM_ROWS: usize = 12;
    const NUM_COLS: usize = 12;
    const WALL_THICKNESS: usize = 2;

    const NORTH_WALL_ROW: usize = 0;
    const SOUTH_WALL_ROW: usize = NUM_ROWS - WALL_THICKNESS;
    const EAST_WALL_COL: usize = NUM_COLS - WALL_THICKNESS;
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
    let mut room_grid = Grid::new(NUM_COLS, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
    ]);

    // Randomly select walls for the exits.
    let mut walls = [Wall::North, Wall::South, Wall::East, Wall::West];
    shuffle_walls(&mut walls);

    for (index, wall) in walls.iter().take(3).enumerate() {
        match index {
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