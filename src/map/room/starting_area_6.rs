use crate::grid::Grid as Grid;
use crate::grid::Node as Node;
use crate::map::room::place_passage as place_passage;
use crate::map::room::Room as Room;
use crate::map::room::Wall as Wall;
use crate::grid::tile::Tile as Tile;
use crate::grid::tile::TileIcon as TileIcon;
use crate::grid::tile::TileKind as TileKind;

// https://github.com/Ronatos/rungeon/wiki/Room#starting-area-6
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

    let mut starting_area6 = Grid::new(8, vec![
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), floor.clone(),floor.clone(),floor.clone(),floor.clone(),wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), floor.clone(),floor.clone(),wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
        wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(), wall.clone(),
    ]);

    // Possibility of 5ft passage removed due to creation of impassable passages
    // 10ft passages do just fine on their own
    starting_area6 = place_passage(starting_area6, Wall::North, 10);
    starting_area6 = place_passage(starting_area6, Wall::South, 10);
    starting_area6 = place_passage(starting_area6, Wall::East, 10);
    starting_area6 = place_passage(starting_area6, Wall::West, 10);
    Room::new(starting_area6, exits)
}
