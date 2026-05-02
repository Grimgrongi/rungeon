use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::Room;
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
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(),wall.clone(), //22 23
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(),wall.clone(), //30 31
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(),wall.clone(), //38 39
        wall.clone(), wall.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), wall.clone(),wall.clone(), //46 47
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone()
    ]);

    // Generate exits for the room.
    let exits = vec![
        Exit::new(
            ExitWall::North,
            ExitKind::Passage,
            match dice::roll(14) {
                1..=2 => ExitWidth::Five,
                3..=12 => ExitWidth::Ten,
                13..=14 => ExitWidth::Twenty,
                _ => panic!("Unexpected dice roll.")
            }
        ),
        Exit::new(
            ExitWall::South,
            ExitKind::Passage,
            match dice::roll(14) {
                1..=2 => ExitWidth::Five,
                3..=12 => ExitWidth::Ten,
                13..=14 => ExitWidth::Twenty,
                _ => panic!("Unexpected dice roll.")
            }
        ),
        Exit::new(
            ExitWall::East,
            ExitKind::Passage,
            match dice::roll(14) {
                1..=2 => ExitWidth::Five,
                3..=12 => ExitWidth::Ten,
                13..=14 => ExitWidth::Twenty,
                _ => panic!("Unexpected dice roll.")
            }
        ),
        Exit::new(
            ExitWall::West,
            ExitKind::Passage,
            match dice::roll(14) {
                1..=2 => ExitWidth::Five,
                3..=12 => ExitWidth::Ten,
                13..=14 => ExitWidth::Twenty,
                _ => panic!("Unexpected dice roll.")
            }
        )
    ];

    // Modify the base shape of the room to accommodate the exits.
    let mut passage_index1: usize;
    let mut passage_index2: usize;
    for exit in exits.iter() {
        if exit.wall == ExitWall::North && exit.width == ExitWidth::Five {
            passage_index1 = (dice::roll(4) + 1) as usize;
            modify_shape_for_passage(&mut shape, ExitWall::North, passage_index1, passage_index1);
        }
        if exit.wall == ExitWall::North && exit.width == ExitWidth::Ten {
            passage_index1 = (dice::roll(3) + 1) as usize;
            modify_shape_for_passage(&mut shape, ExitWall::North, passage_index1, passage_index1 + 1);
        }
        if exit.wall == ExitWall::North && exit.width == ExitWidth::Twenty {
            modify_shape_for_passage(&mut shape, ExitWall::North, 2 as usize, 5 as usize);
        }
        if exit.wall == ExitWall::South && exit.width == ExitWidth::Five {
            passage_index1 = (48 + (dice::roll(4) + 1)) as usize;
            modify_shape_for_passage(&mut shape, ExitWall::South, passage_index1, passage_index1);
        }
        if exit.wall == ExitWall::South && exit.width == ExitWidth::Ten {
            passage_index1 = (48 + (dice::roll(3) + 1)) as usize;
            modify_shape_for_passage(&mut shape, ExitWall::South, passage_index1, passage_index1 + 1);
        }
        if exit.wall == ExitWall::South && exit.width == ExitWidth::Twenty {
            modify_shape_for_passage(&mut shape, ExitWall::South, 50 as usize, 53 as usize);
        }
        if exit.wall == ExitWall::East && exit.width == ExitWidth::Five {
            passage_index1 = (14 + (dice::roll(4) * 8)) as usize;
            shape.nodes[passage_index1] = flor.clone();
            shape.nodes[passage_index1 + 1] = flor.clone();
        }
        if exit.wall == ExitWall::East && exit.width == ExitWidth::Ten {
            passage_index1 = (14 + (dice::roll(3) * 8)) as usize;
            shape.nodes[passage_index1] = flor.clone();
            shape.nodes[passage_index1 + shape.columns] = flor.clone();
            shape.nodes[passage_index1 + 1] = flor.clone();
            shape.nodes[passage_index1 + shape.columns + 1] = flor.clone();
        }
        if exit.wall == ExitWall::East && exit.width == ExitWidth::Twenty {
            shape.nodes[22 as usize] = flor.clone();
            shape.nodes[23 as usize] = flor.clone();
            shape.nodes[30 as usize] = flor.clone();
            shape.nodes[31 as usize] = flor.clone();
            shape.nodes[38 as usize] = flor.clone();
            shape.nodes[39 as usize] = flor.clone();
            shape.nodes[46 as usize] = flor.clone();
            shape.nodes[47 as usize] = flor.clone();
        }
    }

    Room::new(shape, exits)
}

fn modify_shape_for_passage(shape: &mut Grid, wall: ExitWall, passage_index1: usize, passage_index2: usize) {
    let floor_tile = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    if wall == ExitWall::North || wall == ExitWall::South {
        for i in passage_index1..=passage_index2 {
            shape.nodes[i] = floor_tile.clone();
            shape.nodes[i + shape.columns] = floor_tile.clone();
        }
    }
    if wall == ExitWall::East || wall == ExitWall::West {
        for i in passage_index1..=passage_index2 {
            shape.nodes[i] = floor_tile.clone();
            shape.nodes[i + 1] = floor_tile.clone();
        }
    }
}