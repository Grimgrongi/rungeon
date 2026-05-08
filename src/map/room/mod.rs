use std::fmt;

use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};

pub mod starting_area_1;
pub mod starting_area_2;
pub mod starting_area_3;
pub mod starting_area_4;

#[derive(Clone)]
pub struct Room {
    pub grid: Grid
}

impl Room {
    pub fn new(grid: Grid) -> Room {
        Room {
            grid
        }
    }
}

// For testing only
impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExitLength {
    Five,
    Ten,
    Twenty,
    Thirty,
    Forty
}

#[derive(Copy, Clone, PartialEq)]
pub enum Wall {
    North,
    South,
    East,
    West
}

pub fn get_random_exit_length(max_length: ExitLength) -> ExitLength {
    match max_length {
        ExitLength::Five => ExitLength::Five,
        ExitLength::Ten => {
            match dice::roll(12) {
                1..=2 => ExitLength::Five,
                3..=12 => ExitLength::Ten,
                _ => panic!("Unexpected dice roll.")
            }
        },
        ExitLength::Twenty => {
            match dice::roll(14) {
                1..=2 => ExitLength::Five,
                3..=12 => ExitLength::Ten,
                13..=14 => ExitLength::Twenty,
                _ => panic!("Unexpected dice roll.")
            }
        },
        ExitLength::Thirty => {
            match dice::roll(16) {
                1..=2 => ExitLength::Five,
                3..=12 => ExitLength::Ten,
                13..=14 => ExitLength::Twenty,
                15..=16 => ExitLength::Thirty,
                _ => panic!("Unexpected dice roll.")
            }
        },
        ExitLength::Forty => {
            match dice::roll(20) {
                1..=2 => ExitLength::Five,
                3..=12 => ExitLength::Ten,
                13..=14 => ExitLength::Twenty,
                15..=16 => ExitLength::Thirty,
                17..=20 => ExitLength::Forty,
                _ => panic!("Unexpected dice roll.")
            }
        }
    }
}

pub fn get_exit_passage(wall: Wall, width: ExitLength) -> Grid {
    let flor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    let num_columns = if wall == Wall::East || wall == Wall::West {
        2
    }
    else {
        match width {
            ExitLength::Five => 1,
            ExitLength::Ten => 2,
            ExitLength::Twenty => 4,
            ExitLength::Thirty => 6,
            ExitLength::Forty => 8
        }
    };

    Grid::new(
        num_columns,
        match width {
            ExitLength::Five => vec![
                flor.clone(),
                flor.clone()
            ],
            ExitLength::Ten => vec![
                flor.clone(), flor.clone(),
                flor.clone(), flor.clone()
            ],
            ExitLength::Twenty => vec![
                flor.clone(), flor.clone(), flor.clone(), flor.clone(),
                flor.clone(), flor.clone(), flor.clone(), flor.clone()
            ],
            ExitLength::Thirty => vec![
                flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(),
                flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone()
            ],
            ExitLength::Forty => vec![
                flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(),
                flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone(), flor.clone()
            ]
        }
    )
}

pub fn shuffle_walls(walls: &mut [Wall; 4]) {
    for i in (1..walls.len()).rev() {
        let j = (dice::roll((i + 1) as u8) as usize) - 1;
        walls.swap(i, j);
    }
}

pub fn get_exit_door(wall: Wall) -> Grid {
    let door = Node::Tile(Tile {
        kind: TileKind::Door,
        icon: TileIcon::Door
    });
    let flor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });

    let num_columns = if wall == Wall::East || wall == Wall::West {
        2
    }
    else {
        1
    };

    Grid::new(
        num_columns,
        match wall {
            Wall::North | Wall::West => vec![flor.clone(), door.clone()],
            Wall::South | Wall::East => vec![door.clone(), flor.clone()]
        }
    )
}
