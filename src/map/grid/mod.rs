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

    pub fn rows(&self) -> usize {
        self.nodes.len() / self.columns
    }

    pub fn update(&mut self, source: &Grid, target_row: usize, target_column: usize) {
        if target_column + source.columns > self.columns || target_row + source.rows() > self.rows() {
            panic!(
                "Source grid does not fit into target grid at ({}, {})",
                target_row, target_column
            );
        }

        for source_row in 0..source.rows() {
            let dest_base = (target_row + source_row) * self.columns + target_column;
            let source_base = source_row * source.columns;

            for source_column in 0..source.columns {
                self.nodes[dest_base + source_column] = source.nodes[source_base + source_column].clone();
            }
        }
    }

    pub fn rotate_clockwise(&self) -> Grid {
        let mut rotated_nodes: Vec<Node> = Vec::new();
        for column in 0..self.columns {
            for row in (0..self.rows()).rev() {
                let index = row * self.columns + column;
                rotated_nodes.push(self.nodes[index].clone());
            }
        }
        Grid::new(self.rows(), rotated_nodes)
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