pub mod chamber_1;
pub mod chamber_2;
pub mod chamber_3;
pub mod chamber_4;
pub mod chamber_5;
pub mod chamber_6;
pub mod chamber_7;
pub mod chamber_8;
pub mod chamber_9;
pub mod chamber_10;
pub mod chamber_11;
pub mod chamber_12;
pub mod passage_1;
pub mod passage_2;
pub mod passage_3;
pub mod passage_4;
pub mod passage_5;
pub mod passage_6;
pub mod passage_7;
pub mod passage_8;
pub mod passage_9;
pub mod starting_area_1;
pub mod starting_area_2;
pub mod starting_area_3;
pub mod starting_area_4;
pub mod starting_area_5;
pub mod starting_area_6;
pub mod starting_area_7;
pub mod starting_area_8;
pub mod starting_area_9;
pub mod starting_area_10;

use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::grid::tile::Tile as Tile;
use crate::grid::tile::TileIcon as TileIcon;
use crate::grid::tile::TileKind as TileKind;

use rand::Rng;

#[derive(Clone, PartialEq)]
pub enum Wall {
    North,
    South,
    East,
    West
}

#[derive(Clone)]
pub struct Room {
    pub grid: Grid,
    pub exits: Vec<Wall>
}

impl Room {
    pub fn new(grid: Grid, exits: Vec<Wall>) -> Room {
        Room {
            grid,
            exits
        }
    }
}

pub fn place_passage(mut starting_area: Grid, wall: Wall, width: usize) -> Grid {
    let mut rng = rand::thread_rng();
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    match wall {
        Wall::North => {
            let top_row = 0;
            let bottom_row = 1;
            let wall_length = starting_area.columns - 4;
            
            if width == 5 {
                let column = rng.gen_range(2..wall_length + 2);
                starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
                starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
            }
            else if width == 10 {
                let left_column = rng.gen_range(2..wall_length + 1);
                starting_area.nodes[left_column + (starting_area.columns * top_row)] = floor.clone();
                starting_area.nodes[left_column + (starting_area.columns * bottom_row)] = floor.clone();
                starting_area.nodes[(left_column + 1) + (starting_area.columns * top_row)] = floor.clone();
                starting_area.nodes[(left_column + 1) + (starting_area.columns * bottom_row)] = floor.clone();
            }
        },
        Wall::South => {
            let top_row = (starting_area.nodes.len() / starting_area.columns) - 2;
            let bottom_row = (starting_area.nodes.len() / starting_area.columns) - 1;
            let wall_length = starting_area.columns - 4;
            
            if width == 5 {
                let column = rng.gen_range(2..wall_length + 2);
                starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
                starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
            }
            else if width == 10 {
                let left_column = rng.gen_range(2..wall_length + 1);
                starting_area.nodes[left_column + (starting_area.columns * top_row)] = floor.clone();
                starting_area.nodes[left_column + (starting_area.columns * bottom_row)] = floor.clone();
                starting_area.nodes[(left_column + 1) + (starting_area.columns * top_row)] = floor.clone();
                starting_area.nodes[(left_column + 1) + (starting_area.columns * bottom_row)] = floor.clone();
            }
        },
        Wall::East => {
            let left_column = 0;
            let right_column = 1;
            let wall_length = (starting_area.nodes.len() / starting_area.columns) - 4;

            if width == 5 {
                let row = rng.gen_range(2..wall_length + 2);
                starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
                starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
            }
            else if width == 10 {
                let top_row = rng.gen_range(2..wall_length + 1);
                starting_area.nodes[(top_row * starting_area.columns) + left_column] = floor.clone();
                starting_area.nodes[(top_row * starting_area.columns) + right_column] = floor.clone();
                starting_area.nodes[((top_row + 1) * starting_area.columns) + left_column] = floor.clone();
                starting_area.nodes[((top_row + 1) * starting_area.columns) + right_column] = floor.clone();
            }
        },
        Wall::West => {
            let left_column = starting_area.columns - 2;
            let right_column = starting_area.columns - 1;
            let wall_length = (starting_area.nodes.len() / starting_area.columns) - 4;

            if width == 5 {
                let row = rng.gen_range(2..wall_length + 2);
                starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
                starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
            }
            else if width == 10 {
                let top_row = rng.gen_range(2..wall_length + 1);
                starting_area.nodes[(top_row * starting_area.columns) + left_column] = floor.clone();
                starting_area.nodes[(top_row * starting_area.columns) + right_column] = floor.clone();
                starting_area.nodes[((top_row + 1) * starting_area.columns) + left_column] = floor.clone();
                starting_area.nodes[((top_row + 1) * starting_area.columns) + right_column] = floor.clone();
            }
        }
    }
    starting_area
}

pub fn place_door(mut starting_area: Grid, wall: Wall) -> Grid {
    let mut rng = rand::thread_rng();
    let door = Node::Tile(Tile {kind: TileKind::Door, icon: TileIcon::Door});
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Floor});

    match wall {
        Wall::North => {
            let top_row = 0;
            let bottom_row = 1;
            let wall_length = starting_area.columns - 4;
            let column = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
            starting_area.nodes[column + (starting_area.columns * bottom_row)] = door.clone();
        },
        Wall::South => {
            let top_row = (starting_area.nodes.len() / starting_area.columns) - 2;
            let bottom_row = (starting_area.nodes.len() / starting_area.columns) - 1;
            let wall_length = starting_area.columns - 4;
            let column = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[column + (starting_area.columns * top_row)] = door.clone();
            starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
        },
        Wall::East => {
            let left_column = 0;
            let right_column = 1;
            let wall_length = (starting_area.nodes.len() / starting_area.columns) - 4;
            let row = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
            starting_area.nodes[(row * starting_area.columns) + right_column] = door.clone();
        },
        Wall::West => {
            let left_column = starting_area.columns - 2;
            let right_column = starting_area.columns - 1;
            let wall_length = (starting_area.nodes.len() / starting_area.columns) - 4;
            let row = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[(row * starting_area.columns) + left_column] = door.clone();
            starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
        }
    }
    starting_area
}

pub fn place_secret_door(mut starting_area: Grid, wall: Wall) -> Grid {
    let mut rng = rand::thread_rng();
    let door = Node::Tile(Tile {kind: TileKind::Door, icon: TileIcon::Wall});
    let floor = Node::Tile(Tile {kind: TileKind::Floor, icon: TileIcon::Wall});

    match wall {
        Wall::North => {
            let top_row = 0;
            let bottom_row = 1;
            let wall_length = starting_area.columns - 4;
            let column = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[column + (starting_area.columns * top_row)] = floor.clone();
            starting_area.nodes[column + (starting_area.columns * bottom_row)] = door.clone();
        },
        Wall::South => {
            let top_row = (starting_area.nodes.len() / starting_area.columns) - 2;
            let bottom_row = (starting_area.nodes.len() / starting_area.columns) - 1;
            let wall_length = starting_area.columns - 4;
            let column = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[column + (starting_area.columns * top_row)] = door.clone();
            starting_area.nodes[column + (starting_area.columns * bottom_row)] = floor.clone();
        },
        Wall::East => {
            let left_column = 0;
            let right_column = 1;
            let wall_length = (starting_area.nodes.len() / starting_area.columns) - 4;
            let row = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[(row * starting_area.columns) + left_column] = floor.clone();
            starting_area.nodes[(row * starting_area.columns) + right_column] = door.clone();
        },
        Wall::West => {
            let left_column = starting_area.columns - 2;
            let right_column = starting_area.columns - 1;
            let wall_length = (starting_area.nodes.len() / starting_area.columns) - 4;
            let row = rng.gen_range(2..wall_length + 2);
            starting_area.nodes[(row * starting_area.columns) + left_column] = door.clone();
            starting_area.nodes[(row * starting_area.columns) + right_column] = floor.clone();
        }
    }
    starting_area
}