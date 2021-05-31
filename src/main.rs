use std::io;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use time::Instant;

fn get_random_vec(length: usize) -> Vec<u64> {
    let mut small_rng = SmallRng::from_entropy();

    let mut result: Vec<u64> = Vec::with_capacity(length );
    let mut i :usize = 0;
    while i<= length {
        result.push( small_rng.gen());
        i = i + 1;
    }
    return result;
}

fn main() {
    println!("Hello, world!");

    let size = usize::pow(2, 24);
    let start = Instant::now();
    get_random_vec(size).sort_unstable();
    let end = Instant::now();

    println!("{} ms for generating {} integers.", (end - start).whole_milliseconds(), size);
}
