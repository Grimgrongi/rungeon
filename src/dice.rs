use rand::RngExt;

pub fn roll(d: u8) -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=d)
}

pub fn roll_range(min: u8, max: u8) -> u8 {
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}