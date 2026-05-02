#[derive(Clone, PartialEq)]
pub struct Exit {
    pub kind: ExitKind,
    pub wall: ExitWall,
    pub width: ExitWidth
}

#[derive(Clone, PartialEq)]
pub enum ExitKind {
    Door,
    Passage,
    SecretDoor
}

#[derive(Clone, PartialEq)]
pub enum ExitWall {
    North,
    South,
    East,
    West
}

#[derive(Clone, PartialEq)]
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