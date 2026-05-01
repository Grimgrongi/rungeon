use crate::dice;
use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::map::room::place_passage as place_passage;
use crate::map::room::Room as Room;
use crate::map::room::Wall as Wall;
use crate::grid::tile::Tile as Tile;
use crate::grid::tile::TileIcon as TileIcon;
use crate::grid::tile::TileKind as TileKind;

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-1
pub fn new() -> Room {
    let wall = Node::Tile(Tile {
        kind: TileKind::Wall,
        icon: TileIcon::Wall
    });
    let floor = Node::Tile(Tile {
        kind: TileKind::Floor,
        icon: TileIcon::Floor
    });
    let exits = vec![Wall::North, Wall::South, Wall::East, Wall::West];

    let mut starting_area1 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),wall.clone()
    ]);

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::North, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::North, 10);
    }

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::South, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::South, 10);
    }

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::East, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::East, 10);
    }

    if dice::roll(12) <= 2 {
        starting_area1 = place_passage(starting_area1, Wall::West, 5);
    }
    else {
        starting_area1 = place_passage(starting_area1, Wall::West, 10);
    }

    Room::new(starting_area1, exits)
}