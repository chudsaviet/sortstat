mod sortstat;

use crate::sortstat::estimator::{Complexity, Estimator};
use crate::sortstat::sorter_trait::Sorter;
use crate::sortstat::std_sorter::StdSorter;
use crate::sortstat::std_sorter_unstable::StdSorterUnstable;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use time::Instant;
use crate::sortstat::bubble_sorter::BubbleSorter;
use crate::sortstat::merge_sorter::MergeSorter;
use crate::sortstat::select_sorter::SelectSorter;
use crate::sortstat::quick_sorter::QuickSorter;

const EST_START: usize = usize::pow(2, 14);
const EST_MIN_START_DURATION_MS: i128 = 1000;
const EST_MULTIPLIER: f32 = 1.2;
const EST_ITERATIONS: u32 = 5;

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

fn is_sorted(vec: Vec<u64>) -> bool {
    let mut prev_element = u64::MIN;
    for element in vec {
        if element < prev_element {
            return false;
        }
        prev_element = element;

    }
    return true;
}

fn bench(sorter: &mut dyn Sorter, vec_size: usize) -> i128 {
    let mut vec = get_random_vec(vec_size);

    let start = Instant::now();
    sorter.sort(&mut vec);

    let duration = (Instant::now() - start).whole_milliseconds();

    if !is_sorted(vec) {
        println!("Sort is incorrect!");
        panic!();
    }

    duration
}

fn estimate(sorter: &mut dyn Sorter) -> Complexity {
    let mut stage_time_ms = bench(sorter, EST_START);
    let mut vec_size: usize = EST_START;
    while stage_time_ms < EST_MIN_START_DURATION_MS {
        vec_size = vec_size * 2;
        stage_time_ms = bench(sorter, vec_size);
    }

    let mut estimator = Estimator::new(vec_size, stage_time_ms);
    for _ in 0..EST_ITERATIONS {
        stage_time_ms = bench(sorter, vec_size);
        estimator.ingest(vec_size, stage_time_ms);

        vec_size = ((vec_size as f32) * EST_MULTIPLIER) as usize;

        println!("{} elements: {} ms", vec_size, stage_time_ms);
    }

    estimator.get_closest_complexity()
}

fn main() {
    println!("Testing StdSorter...");
    let mut sorter = Box::new(StdSorter {});
    let estimation = estimate(sorter.as_mut());
    println!("StdSorter is close to {}", estimation);

    println!("Testing StdSorterUnstable...");
    let mut sorter = Box::new(StdSorterUnstable {});
    let estimation = estimate(sorter.as_mut());
    println!("StdSorterUnstable close to {}", estimation);

    println!("Testing SelectSorter...");
    let mut sorter = Box::new(SelectSorter {});
    let estimation = estimate(sorter.as_mut());
    println!("SelectSorter close to {}", estimation);

    println!("Testing BubbleSorter...");
    let mut sorter = Box::new(BubbleSorter {});
    let estimation = estimate(sorter.as_mut());
    println!("BubbleSorter close to {}", estimation);

    println!("Testing MergeSorter...");
    let mut sorter = Box::new(MergeSorter {});
    let estimation = estimate(sorter.as_mut());
    println!("MergeSorter close to {}", estimation);

    println!("Testing QuickSorter...");
    let mut sorter = Box::new(QuickSorter {});
    let estimation = estimate(sorter.as_mut());
    println!("QuickSorter close to {}", estimation);

}
