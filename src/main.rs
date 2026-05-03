mod dice;
mod map;

fn main() {
    let room = map::room::starting_area_2::new();
    println!("{}", room);
}