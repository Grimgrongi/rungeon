pub mod grid;
pub mod room;

// use crate::dice;
// use crate::grid::Grid as Grid;
// use crate::grid::Node as Node;
// use crate::map::room::Room as Room;
// use crate::map::room::Wall as Wall;

// enum RoomGenerated {
//     True(Room),
//     False
// }

// pub fn new(num_rows: usize, num_columns: usize, starting_area_index: usize) -> Grid {
//     let min_index: usize = 0;
//     let max_index: usize = num_rows * num_columns;

//     if num_rows == 0 || num_columns == 0 {
//         panic!("Map cannot be generated with {} rows and {} columns.", num_rows, num_columns);
//     }
//     if starting_area_index > max_index { // usize cannot be < 0, so check is not needed
//         panic!("Map cannot be generated. starting_area_index {} is out of bounds.", starting_area_index);
//     }

//     let mut checklist: Vec<RoomGenerated> = Vec::new();
//     for _i in min_index..max_index {
//         checklist.push(RoomGenerated::False);
//     }
//     /* 
//         checklist = [
//             false, false, false,
//             false, false, false,
//             false, false, false,
//         ]
//     */

//     // Every map needs a starting area, so we can safely generate this first.
//     let starting_area_configuration = dice::roll(10);
//     let starting_area: Room = match starting_area_configuration {
//         // 1 => room::starting_area_1::new(),
//         // 2 => room::starting_area_2::new(),
//         // 3 => room::starting_area_3::new(),
//         // 4 => room::starting_area_4::new(),
//         // 5 => room::starting_area_5::new(),
//         // 6 => room::starting_area_6::new(),
//         // 7 => room::starting_area_7::new(),
//         // 8 => room::starting_area_8::new(),
//         // 9 => room::starting_area_9::new(),
//         // 10 => room::starting_area_10::new(),
//         _ => room::starting_area_1::new()
//     };

//     checklist[starting_area_index] = RoomGenerated::True(starting_area.clone());

//     /* 
//         With the starting area done, our checklist could look like this
//         checklist = [
//             false, false, false,
//             false, true(something), false,
//             false, false, false,
//         ]
//     */

//     // This is where the meat of generating the map really happens.
//     let mut rooms_left_to_generate: usize = max_index - 1;
//     let mut room_index = starting_area_index;
//     let mut room_indexes = vec![room_index]; // Keep track of the room_indexes of rooms we need to handle exits for as we go
//     while rooms_left_to_generate > 0 {

//         if let RoomGenerated::True(room) = &checklist[room_index] {
            
//             if (room.exits.contains(&Wall::North)) // The room we're looking at has a northern exit
//                 && (room_index - num_columns >= min_index) // The room to the north can't go out of bounds
//                 && room_indexes.contains(&(room_index - num_columns)) // We can't place a room to the north if another room has claimed that space
//                 && match &checklist[room_index - num_columns] {
//                     RoomGenerated::True(_room) => false,
//                     RoomGenerated::False => true // We can't place a room to the north if one is already there
//                 } {
                
//                 // Build the North room
//                 //     We will need to pass the room builder 2 parameters: we need it to connect to the South, and the passage width.
//                 // Clone the room to the checklist at the room_index
//                 //     checklist[room_index] = RoomGenerated::True(room_type.clone());
//                 // Add the North room's room_index to room_indexes, because we now need to do that one too
//                 //     room_indexes.push(room_index - num_columns);

//             }

//             if (room.exits.contains(&Wall::South)) // The room we're looking at has a southern exit
//                 && (room_index + num_columns <= max_index) // The room to the south can't go out of bounds
//                 && room_indexes.contains(&(room_index + num_columns)) // We can't place a room to the south if another room has claimed that space
//                 && match &checklist[room_index + num_columns] {
//                     RoomGenerated::True(_room) => false,
//                     RoomGenerated::False => true // We can't place a room to the south if one is already there
//                 } {
                
//                 // Build the South room
//                 //     We will need to pass the room builder 2 parameters: we need it to connect to the North, and the passage width.
//                 // Clone the room to the checklist at the room_index
//                 //     checklist[room_index] = RoomGenerated::True(room_type.clone());
//                 // Add the South room's room_index to room_indexes, because we now need to do that one too
//                 //     room_indexes.push(room_index - num_columns);

//             }

//             if (room.exits.contains(&Wall::East)) // The room we're looking at has an eastern exit
//                 && (room_index + 1 <= max_index) // The room to the east can't go out of bounds
//                 && room_indexes.contains(&(room_index + 1)) // We can't place a room to the east if another room has claimed that space
//                 && match &checklist[room_index + 1] {
//                     RoomGenerated::True(_room) => false,
//                     RoomGenerated::False => true // We can't place a room to the east if one is already there
//                 }
//                 && !(room_index + 1 == 0 || (room_index + 1) % num_columns == 0) { // We can't build a room to the east if the next index is on the west
                
//                 // Build the East room
//                 //     We will need to pass the room builder 2 parameters: we need it to connect to the West, and the passage width.
//                 // Clone the room to the checklist at the room_index
//                 //     checklist[room_index] = RoomGenerated::True(room_type.clone());
//                 // Add the East room's room_index to room_indexes, because we now need to do that one too
//                 //     room_indexes.push(room_index - num_columns);

//             }

//             if (room.exits.contains(&Wall::West)) // The room we're looking at has a western exit
//                 && (room_index - 1 >= min_index) // The room to the west can't go out of bounds
//                 && room_indexes.contains(&(room_index - 1)) // We can't place a room to the west if another room has claimed that space
//                 && match &checklist[room_index - 1] {
//                     RoomGenerated::True(_room) => false,
//                     RoomGenerated::False => true // We can't place a room to the west if one is already there
//                 }
//                 && !(room_index % num_columns == 0 || room_index == min_index) { // We can't build a room to the west if the previous index is on the east
                
//                 // Build the West room
//                 //     We will need to pass the room builder 2 parameters: we need it to connect to the East, and the passage width.
//                 // Clone the room to the checklist at the room_index
//                 //     checklist[room_index] = RoomGenerated::True(room_type.clone());
//                 // Add the West room's room_index to room_indexes, because we now need to do that one too
//                 //     room_indexes.push(room_index - num_columns);

//             }
//         }

//         // Room at room_index is complete
//         room_indexes.remove(room_index);
//         room_index = room_indexes[0];
//         rooms_left_to_generate -= 1;
//     }

//     // The checklist was convenient, but now we need to load in the proper values to a vector in the correct order.
//     let mut map_vector: Vec<Node> = Vec::new();
//     for i in &checklist {
//         if let RoomGenerated::True(room) = i {
//             map_vector.push(Node::Room(room.grid.clone()));
//         };
//     }
    
//     let map: Grid = Grid::new(1, map_vector);
//     return map;
// }