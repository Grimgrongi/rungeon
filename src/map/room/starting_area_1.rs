use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::{Room, placement_on_wall};
use crate::map::room::{add_passage, get_random_exit_width};
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
    const MAX_EXIT_WIDTH: ExitWidth = ExitWidth::Twenty;
    let exits = vec![
        Exit::new(ExitWall::North, ExitKind::Passage, get_random_exit_width(MAX_EXIT_WIDTH)),
        Exit::new(ExitWall::South, ExitKind::Passage, get_random_exit_width(MAX_EXIT_WIDTH)),
        Exit::new(ExitWall::East, ExitKind::Passage, get_random_exit_width(MAX_EXIT_WIDTH)),
        Exit::new(ExitWall::West, ExitKind::Passage, get_random_exit_width(MAX_EXIT_WIDTH))
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
