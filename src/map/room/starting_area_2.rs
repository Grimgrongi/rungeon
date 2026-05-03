use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::Room;
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
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    let mut exits: Vec<Wall> = Vec::new();

    let mut starting_area2 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone()
    ]);

    // Repeat exactly 3 times.
    // 1. Select a wall from the remaining empty walls at random.
    // 2. Determine if a passage or door is to be built
    // 2a. Build a passage
    // 2b. Build a door
    // 3. Remove whichever wall is selected from the list of available walls.

    let mut exits_to_build = 3;
    let mut num_doors = 0;
    let mut num_passages = 0;
    let mut empty_walls = vec![Wall::North, Wall::South, Wall::East, Wall::West];
    let mut rng = rand::thread_rng();
    while exits_to_build > 0 {
        let wall_index = rng.gen_range(0..empty_walls.len());
        let wall_selection = &empty_walls[wall_index];

        if num_passages != 1 {
            match wall_selection {
                Wall::North => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::North, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::North, 10);
                    }
                    exits.push(Wall::North);
                },
                Wall::South => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::South, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::South, 10);
                    }
                    exits.push(Wall::South);
                },
                Wall::East => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::East, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::East, 10);
                    }
                    exits.push(Wall::East);
                },
                Wall::West => {
                    if dice::roll(12) <= 2 { // This will be a 5ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::West, 5);
                    }
                    else { // This will be a 10ft wide passage
                        starting_area2 = place_passage(starting_area2, Wall::West, 10);
                    }
                    exits.push(Wall::West);
                }
            }
            num_passages = num_passages + 1;
        }
        else if num_doors != 2 {
            match wall_selection {
                Wall::North => {
                    starting_area2 = place_door(starting_area2, Wall::North);
                    exits.push(Wall::North);
                },
                Wall::South => {
                    starting_area2 = place_door(starting_area2, Wall::South);
                    exits.push(Wall::South);
                },
                Wall::East => {
                    starting_area2 = place_door(starting_area2, Wall::East);
                    exits.push(Wall::East);
                },
                Wall::West => {
                    starting_area2 = place_door(starting_area2, Wall::West);
                    exits.push(Wall::West);
                }
            }
            num_doors = num_doors + 1;
        }

        empty_walls.remove(wall_index);
        exits_to_build = exits_to_build - 1;
    }

    Room::new(starting_area2, exits)
}