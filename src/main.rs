mod dice;
mod map;

fn main() {
    let room1 = map::room::starting_area_1::new();
    let room2 = map::room::starting_area_2::new();
    println!("{}", room1);
    println!("{}", room2);
}