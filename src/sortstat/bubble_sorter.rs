use crate::sortstat::sorter_trait::Sorter;

pub struct BubbleSorter {}

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
