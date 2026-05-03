mod dice;
mod map;

fn main() {
    let room = map::room::starting_area_1::new();
    println!("{}", room);
}