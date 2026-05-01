// https://github.com/Grimgrongi/rungeon/wiki/Grid

pub mod tile;
use std::fmt;

#[derive(Clone)]
pub struct Grid {
    pub columns: usize,
    pub nodes: Vec<Node>
}

impl Grid {
    pub fn new(num_columns: usize, nodes: Vec<Node>) -> Grid {
        let num_nodes = nodes.len();
        if num_nodes % num_columns != 0 {
            panic!("Grid with {} columns is incompatible with vector length {}.", num_columns, num_nodes);
        }

        Grid {
            columns: num_columns,
            nodes
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, node) in self.nodes.iter().enumerate() {
            if (i) % self.columns == 0 {
                write!(f, "{}", "\n")?;
            }
            write!(f, "{}", node)?;
        }
        write!(f, "{}", "")
    }
}

#[derive(Clone)]
pub enum Node {
    Room(Grid),
    Tile(tile::Tile)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Room(grid) => write!(f, "{}", grid),
            Node::Tile(tile) => write!(f, "{}", tile)
        }
    }
}