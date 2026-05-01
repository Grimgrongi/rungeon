// https://github.com/Grimgrongi/rungeon/wiki/Tile

use std::fmt;

#[derive(Copy, Clone)]
pub struct Tile {
    pub kind: TileKind,
    pub icon: TileIcon
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.icon)
    }
}

#[derive(Copy, Clone)]
pub enum TileKind {
    Door,
    Floor,
    Wall,
    Well
}

#[derive(Copy, Clone)]
pub enum TileIcon {
    Door,
    Floor,
    Wall,
    Well
}

impl fmt::Display for TileIcon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileIcon::Door => write!(f, "{} ", "d"),
            TileIcon::Floor => write!(f, "{} ", " "),
            TileIcon::Wall => write!(f, "{} ", "#"),
            TileIcon::Well => write!(f, "{} ", "w"),
        }
    }
}
