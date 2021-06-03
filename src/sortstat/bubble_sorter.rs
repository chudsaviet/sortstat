use crate::sortstat::sorter_trait::Sorter;
use std::borrow::Borrow;

pub struct BubbleSorter {}

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

impl Sorter for BubbleSorter {


    fn sort(&mut self, vec: &mut Vec<u64>) {
        if vec.len() < 2 {
            return;
        }

        let mut i:usize = 1;
        while i < vec.len() {
            if vec[i] < vec[i-1] {
                let buf = vec[i-1];
                vec[i-1] = vec[i];
                vec[i] = buf;

                if i > 1 {
                    i-=1;
                }
            } else {
                i += 1;
            }
        }

    }
}
