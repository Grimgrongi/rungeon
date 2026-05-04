use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, add_door};
use crate::map::room::{add_passage, get_exit_width, shuffle_walls, grid_index, placement_on_wall};
use crate::map::room::exit::{Exit, ExitKind, ExitWall, ExitWidth};

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

    // Generate exits for the room.
    let mut walls = [ExitWall::North, ExitWall::South, ExitWall::East, ExitWall::West];
    shuffle_walls(&mut walls);

    let exits = vec![
        Exit::new(walls[0], ExitKind::Door, ExitWidth::Five),
        Exit::new(walls[1], ExitKind::Door, ExitWidth::Five),
        Exit::new(walls[2], ExitKind::Passage, get_exit_width())
    ];

    // Modify the base shape of the room to accommodate the exits.
    const NORTH_ROW: usize = 0;
    const SOUTH_WALL_OFFSET: u8 = 6 * 8; // 6 rows down * 8 columns per row = 48
    const EAST_WALL_OFFSET: u8 = 14; // 1 row * 8 columns + 6 columns
    for exit in exits.iter() {
        match (exit.wall, exit.kind, exit.width) {
            (ExitWall::North, ExitKind::Door, ExitWidth::Five) => {
                // North wall: rows 0-1, columns 2-5 are valid for a 1 wide passage
                let (row, col, height, width) = placement_on_wall(exit.wall, exit.width);
                add_door(
                    &mut shape,
                    row,
                    col,
                    width,
                    height
                );
            },
            (ExitWall::North, ExitKind::Passage, ExitWidth::Five) => {
                // North wall: rows 0-1, columns 2-5 are valid for a 1 wide passage
                let index = (dice::roll(4) + 1) as usize; // 1-4 + 1 = 2-5
                add_passage(&mut shape, index, 1, 2, columns);
            },
            (ExitWall::North, ExitKind::Passage, ExitWidth::Ten) => {
                // North wall: rows 0-1, columns 2-4 are valid for a 2 wide passage
                let index = (dice::roll(3) + 1) as usize; // 1-3 + 1 = 2-4
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::North, ExitKind::Passage, ExitWidth::Twenty) => {
                // North wall: rows 0-1, columns 2-5 are valid for a 4 wide passage
                let index = 2;
                add_passage(&mut shape, index, 4, 2, columns);
            },
            (ExitWall::South, ExitKind::Door, ExitWidth::Five) => {
                // South wall: rows 6-7, columns 2-5 (50-53) are valid for a 1 wide passage
                let index = ((dice::roll(4) + 1) + SOUTH_WALL_OFFSET) as usize; // 1-4 + 1 + 48 (6 rows down) = 50-53
                add_door(&mut shape, index, 1, 2, columns);
            },
            (ExitWall::South, ExitKind::Passage, ExitWidth::Five) => {
                // South wall: rows 6-7, columns 2-5 (50-53) are valid for a 1 wide passage
                let index = ((dice::roll(4) + 1) + SOUTH_WALL_OFFSET) as usize; // 1-4 + 1 + 48 (6 rows down) = 50-53
                add_passage(&mut shape, index, 1, 2, columns);
            },
            (ExitWall::South, ExitKind::Passage, ExitWidth::Ten) => {
                // South wall: rows 6-7, columns 2-4 (50-52) are valid for a 2 wide passage
                let index = ((dice::roll(3) + 1) + SOUTH_WALL_OFFSET) as usize; // 1-3 + 1 + 48 (6 rows down) = 50-52
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::South, ExitKind::Passage, ExitWidth::Twenty) => {
                // South wall: rows 6-7, columns 2-5 (50-53) are valid for a 4 wide passage
                let index = 50;
                add_passage(&mut shape, index, 4, 2, columns);
            },
            (ExitWall::East, ExitKind::Door, ExitWidth::Five) => {
                // East wall: columns 7-8, rows 2-5 are valid for a 1 wide passage
                let index = ((dice::roll(4) * 8) + EAST_WALL_OFFSET) as usize; // 1-4 * 8 + 14 = 22, 30, 38, 46
                add_door(&mut shape, index, 2, 1, columns);
            },
            (ExitWall::East, ExitKind::Passage, ExitWidth::Five) => {
                // East wall: columns 7-8, rows 2-5 are valid for a 1 wide passage
                let index = ((dice::roll(4) * 8) + EAST_WALL_OFFSET) as usize; // 1-4 * 8 + 14 = 22, 30, 38, 46
                add_passage(&mut shape, index, 2, 1, columns);
            },
            (ExitWall::East, ExitKind::Passage, ExitWidth::Ten) => {
                // East wall: columns 7-8, rows 2-4 are valid for a 2 wide passage
                let index = ((dice::roll(3) * 8) + EAST_WALL_OFFSET) as usize; // 1-3 * 8 + 14 = 22, 30, 38
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::East, ExitKind::Passage, ExitWidth::Twenty) => {
                // East wall: columns 7-8, rows 2-5 are valid for a 4 wide passage
                let index = 22;
                add_passage(&mut shape, index, 2, 4, columns);
            },
            (ExitWall::West, ExitKind::Door, ExitWidth::Five) => {
                // West wall: columns 0-1, rows 2-5 are valid for a 1 wide passage
                let index = ((dice::roll(4) + 1) * 8) as usize; // 1-4 + 1 = 2-5, then multiply by 8 to get the left column of the passage on the west wall = 16, 24, 32, 40
                add_door(&mut shape, index, 2, 1, columns);
            },
            (ExitWall::West, ExitKind::Passage, ExitWidth::Five) => {
                // West wall: columns 0-1, rows 2-5 are valid for a 1 wide passage
                let index = ((dice::roll(4) + 1) * 8) as usize; // 1-4 + 1 = 2-5, then multiply by 8 to get the left column of the passage on the west wall = 16, 24, 32, 40
                add_passage(&mut shape, index, 2, 1, columns);
            },
            (ExitWall::West, ExitKind::Passage, ExitWidth::Ten) => {
                // West wall: columns 0-1, rows 2-4 are valid for a 2 wide passage
                let index = ((dice::roll(3) + 1) * 8) as usize; // 1-3 + 1 = 2-4, then multiply by 8 to get the left column of the passage on the west wall = 16, 24, 32
                add_passage(&mut shape, index, 2, 2, columns);
            },
            (ExitWall::West, ExitKind::Passage, ExitWidth::Twenty) => {
                // West wall: columns 0-1, rows 2-5 are valid for a 4 wide passage
                let index = 16;
                add_passage(&mut shape, index, 2, 4, columns);
            },
            _ => panic!("Unexpected exit configuration.")
        }
    }

    Room::new(shape, exits)
}

