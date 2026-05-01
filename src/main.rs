mod dice;
mod map;

use crate::map::grid::Grid as Grid;
// use crate::map as Map;

fn main() {
    let grid = Grid::new(3, vec![
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Floor,
            icon: map::grid::tile::TileIcon::Floor
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        }),
        map::grid::Node::Tile(map::grid::tile::Tile {
            kind: map::grid::tile::TileKind::Wall,
            icon: map::grid::tile::TileIcon::Wall
        })
    ]);
    println!("{}", grid);

    // let map: Grid = Map::new(1, 1, 0);
    // println!("{}", map);
}