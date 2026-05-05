#[derive(Clone, PartialEq)]
pub struct Exit {
    pub kind: ExitKind,
    pub wall: ExitWall,
    pub width: ExitWidth
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

pub struct ExitPlacement {
    pub row: usize,
    pub column: usize,
    pub width: usize,
    pub height: usize
}

impl ExitPlacement {
    pub fn new(row: usize, column: usize, width: usize, height: usize) -> ExitPlacement {
        ExitPlacement {
            row,
            column,
            width,
            height
        }
    }
}