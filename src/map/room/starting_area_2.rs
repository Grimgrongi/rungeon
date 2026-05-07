use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, ExitLength, get_random_exit_length};

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
    let mut shape = Grid::new(8, vec![
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

    // Generate exits for the room.
    const MAX_EXIT_WIDTH: ExitWidth = ExitWidth::Twenty;
    let mut walls = [ExitWall::North, ExitWall::South, ExitWall::East, ExitWall::West];
    shuffle_walls(&mut walls);

    let exits = vec![
        Exit::new(walls[0], ExitKind::Door, ExitWidth::Five),
        Exit::new(walls[1], ExitKind::Door, ExitWidth::Five),
        Exit::new(walls[2], ExitKind::Passage, get_random_exit_width(MAX_EXIT_WIDTH))
    ];

    // Modify the base shape of the room to accommodate the exits.
    for exit in exits.iter() {
        match exit.wall {
            ExitWall::North => {
                add_passage(&mut shape, placement_on_wall(ExitWall::North, exit.width));
            },
            ExitWall::South => {
                add_passage(&mut shape, placement_on_wall(ExitWall::South, exit.width));
            },
            ExitWall::East => {
                add_passage(&mut shape, placement_on_wall(ExitWall::East, exit.width));
            },
            ExitWall::West => {
                add_passage(&mut shape, placement_on_wall(ExitWall::West, exit.width));
             }
        }
    }

    Room::new(shape, exits)
}

