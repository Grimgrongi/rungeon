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