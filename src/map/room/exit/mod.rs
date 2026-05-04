pub mod exit_placement;

#[derive(Clone, PartialEq)]
pub struct Exit {
    pub kind: ExitKind,
    pub wall: ExitWall,
    pub width: ExitWidth
}

#[derive(Copy, Clone, PartialEq)]
pub enum ExitKind {
    Door,
    Passage,
    SecretDoor
}

#[derive(Copy, Clone, PartialEq)]
pub enum ExitWall {
    North,
    South,
    East,
    West
}

#[derive(Copy, Clone, PartialEq)]
pub enum ExitWidth {
    Five,
    Ten,
    Twenty
}

impl Exit {
    pub fn new(wall: ExitWall, kind: ExitKind, width: ExitWidth) -> Exit {
        Exit {
            kind,
            wall,
            width
        }
    }
}