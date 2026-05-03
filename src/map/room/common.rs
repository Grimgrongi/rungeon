use crate::dice;
use crate::map::grid::{Grid, Node};
use crate::map::grid::tile::{Tile, TileIcon, TileKind};
use crate::map::room::exit::{ExitWall, ExitWidth};

pub fn add_door(shape: &mut Grid, start: usize, width: usize, height: usize, columns: usize) {
    let door = Node::Tile(Tile {
        kind: TileKind::Door,
        icon: TileIcon::Door
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    for h in 0..height {
        for w in 0..width {
            let node = if h == 0 && w == 0 { door.clone() } else { floor.clone() };
            shape.nodes[start + w + (h * columns)] = node;
        }
    }
}

pub fn add_passage(shape: &mut Grid, start: usize, width: usize, height: usize, columns: usize) {
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    for h in 0..height {
        for w in 0..width {
            shape.nodes[start + w + (h * columns)] = floor.clone();
        }
    }
}

pub fn get_exit_width() -> ExitWidth {
    match dice::roll(14) {
        1..=2 => ExitWidth::Five,
        3..=12 => ExitWidth::Ten,
        13..=14 => ExitWidth::Twenty,
        _ => panic!("Unexpected dice roll.")
    }
}

pub fn shuffle_walls(walls: &mut [ExitWall; 4]) {
    for i in (1..walls.len()).rev() {
        let j = (dice::roll((i + 1) as u8) as usize) - 1;
        walls.swap(i, j);
    }
}
