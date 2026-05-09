mod dice;
mod map;

fn main() {
    let room1 = map::room::starting_area_1::new();
    let room2 = map::room::starting_area_2::new();
    let room3 = map::room::starting_area_3::new();
    let room4 = map::room::starting_area_4::new();
    println!("{}", room1);
    println!("{}", room2);
    println!("{}", room3);
    println!("{}", room4);
}