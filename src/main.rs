mod sortstat;

use crate::sortstat::estimator::{Complexity, Estimator};
use crate::sortstat::sorter_trait::Sorter;
use crate::sortstat::std_sorter::StdSorter;
use crate::sortstat::std_sorter_unstable::StdSorterUnstable;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use time::Instant;
use crate::sortstat::bubble_sorter::BubbleSorter;

const EST_START: usize = usize::pow(2, 12);
const EST_MULTIPLIER: f32 = 1.5;
const EST_MAX_TIME_MS: i128 = 16000;

fn get_random_vec(length: usize) -> Vec<u64> {
    let mut small_rng = SmallRng::from_entropy();

    let mut result: Vec<u64> = Vec::with_capacity(length);
    let mut i: usize = 0;
    while i <= length {
        result.push(small_rng.gen());
        i = i + 1;
    }
    return result;
}

fn bench(sorter: &mut dyn Sorter, vec_size: usize) -> i128 {
    let mut vec = get_random_vec(vec_size);

    let start = Instant::now();
    sorter.sort(&mut vec);

    (Instant::now() - start).whole_milliseconds()
}

fn estimate(sorter: &mut dyn Sorter) -> Complexity {
    let mut stage_time_ms = bench(sorter, EST_START);
    let mut estimator = Estimator::new(EST_START, stage_time_ms);

    let mut vec_size: usize = ((EST_START as f32) * EST_MULTIPLIER) as usize;
    while stage_time_ms < EST_MAX_TIME_MS {
        stage_time_ms = bench(sorter, vec_size);
        estimator.ingest(vec_size, stage_time_ms);

        vec_size = ((vec_size as f32) * EST_MULTIPLIER) as usize;

        println!("{}: {} ms", vec_size, stage_time_ms);
    }

    estimator.get_closest_complexity()
}

fn main() {
    let mut sorter = Box::new(StdSorter {});
    let estimation = estimate(sorter.as_mut());
    println!("StdSorter is close to {}", estimation);

    let mut sorter = Box::new(StdSorterUnstable {});
    let estimation = estimate(sorter.as_mut());
    println!("StdSorterUnstable close to {}", estimation);

    let mut sorter = Box::new(BubbleSorter {});
    let estimation = estimate(sorter.as_mut());
    println!("BubbleSorter close to {}", estimation);
}
