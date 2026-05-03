use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::Room;
use crate::map::room::{add_passage, get_exit_width};
use crate::map::room::exit::{Exit, ExitKind, ExitWall, ExitWidth};

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
    let mut shape = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(), //22 23
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(), //30 31
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(), //38 39
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(), wall.clone(), //46 47
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone()
    ]);

    // Generate exits for the room.
    let exits = vec![
        Exit::new(ExitWall::North, ExitKind::Passage, get_exit_width()),
        Exit::new(ExitWall::South, ExitKind::Passage, get_exit_width()),
        Exit::new(ExitWall::East, ExitKind::Passage, get_exit_width()),
        Exit::new(ExitWall::West, ExitKind::Passage, get_exit_width())
    ];

    // Modify the base shape of the room to accommodate the exits.
    const SOUTH_WALL_OFFSET: u8 = 6 * 8; // 6 rows down * 8 columns per row = 48
    const EAST_WALL_OFFSET: u8 = 14; // 1 row * 8 columns + 6 columns
    let columns = shape.columns;
    for exit in exits.iter() {
        match (exit.wall, exit.width) {
            (ExitWall::North, ExitWidth::Five) => {
                // North wall: rows 0-1, columns 2-5 are valid for a 1 wide passage
                let index = (dice::roll(4) + 1) as usize; // 1-4 + 1 = 2-5
                add_passage(&mut shape, index, 1, 2, columns);
            },
            (ExitWall::North, ExitWidth::Ten) => {
                // North wall: rows 0-1, columns 2-4 are valid for a 2 wide passage
                let index = (dice::roll(3) + 1) as usize; // 1-3 + 1 = 2-4
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::North, ExitWidth::Twenty) => {
                // North wall: rows 0-1, columns 2-5 are valid for a 4 wide passage
                let index = 2;
                add_passage(&mut shape, index, 4, 2, columns);
            },
            (ExitWall::South, ExitWidth::Five) => {
                // South wall: rows 6-7, columns 2-5 (50-53) are valid for a 1 wide passage
                let index = ((dice::roll(4) + 1) + SOUTH_WALL_OFFSET) as usize; // 1-4 + 1 + 48 (6 rows down) = 50-53
                add_passage(&mut shape, index, 1, 2, columns);
            },
            (ExitWall::South, ExitWidth::Ten) => {
                // South wall: rows 6-7, columns 2-4 (50-52) are valid for a 2 wide passage
                let index = ((dice::roll(3) + 1) + SOUTH_WALL_OFFSET) as usize; // 1-3 + 1 + 48 (6 rows down) = 50-52
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::South, ExitWidth::Twenty) => {
                // South wall: rows 6-7, columns 2-5 (50-53) are valid for a 4 wide passage
                let index = 50;
                add_passage(&mut shape, index, 4, 2, columns);
            },
            (ExitWall::East, ExitWidth::Five) => {
                // East wall: columns 7-8, rows 2-5 are valid for a 1 wide passage
                let index = ((dice::roll(4) * 8) + EAST_WALL_OFFSET) as usize; // 1-4 * 8 + 14 = 22, 30, 38, 46
                add_passage(&mut shape, index, 2, 1, columns);
            },
            (ExitWall::East, ExitWidth::Ten) => {
                // East wall: columns 7-8, rows 2-4 are valid for a 2 wide passage
                let index = ((dice::roll(3) * 8) + EAST_WALL_OFFSET) as usize; // 1-3 * 8 + 14 = 22, 30, 38
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::East, ExitWidth::Twenty) => {
                // East wall: columns 7-8, rows 2-5 are valid for a 4 wide passage
                let index = 22;
                add_passage(&mut shape, index, 2, 4, columns);
            },
            (ExitWall::West, ExitWidth::Five) => {
                // West wall: columns 0-1, rows 2-5 are valid for a 1 wide passage
                let index = ((dice::roll(4) + 1) * 8) as usize; // 1-4 + 1 = 2-5, then multiply by 8 to get the left column of the passage on the west wall = 16, 24, 32, 40
                add_passage(&mut shape, index, 2, 1, columns);
            },
            (ExitWall::West, ExitWidth::Ten) => {
                // West wall: columns 0-1, rows 2-4 are valid for a 2 wide passage
                let index = ((dice::roll(3) + 1) * 8) as usize; // 1-3 + 1 = 2-4, then multiply by 8 to get the left column of the passage on the west wall = 16, 24, 32
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::West, ExitWidth::Twenty) => {
                // West wall: columns 0-1, rows 2-5 are valid for a 4 wide passage
                let index = 16;
                add_passage(&mut shape, index, 2, 4, columns);
            }
        }
    }

    Room::new(shape, exits)
}
