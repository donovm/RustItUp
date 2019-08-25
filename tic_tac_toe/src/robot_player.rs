extern crate rand;

use rand::Rng;

pub fn get_random_move() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 9)
}
