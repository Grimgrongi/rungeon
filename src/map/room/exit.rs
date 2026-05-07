use std::collections::btree_map::Range;

use crate::dice;

#[derive(Clone, PartialEq)]
pub struct Exit {
    pub kind: ExitKind,
    pub wall: ExitWall,
    pub width: ExitWidth
}

// impl Exit {
//     pub fn new(wall: ExitWall, kind: ExitKind, rows: Range<i32>, columns: Range<i32>) -> Exit {
//         Exit {
//             kind,
//             wall,
//             ExitWidth::Five
//         }
//     }
// }

#[derive(Copy, Clone, PartialEq)]
pub enum ExitKind {
    Door,
    Passage,
    SecretDoor
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