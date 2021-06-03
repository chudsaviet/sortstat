use crate::sortstat::sorter_trait::Sorter;

pub struct MergeSorter {}

impl MergeSorter {
    fn merge_sort(slice: &mut [u64], level: u32) {
        /*for _ in 1..level {
            print!(" ");
        }
        println!("merge_sort({:?})", slice);*/
        let len = slice.len();
        if len < 2 {
            return;
        } else if len == 2 {
            if slice[0] > slice[1] {
                let buf = slice[0];
                slice[0] = slice[1];
                slice[1] = buf;
            }
        } else {
            MergeSorter::merge_sort(&mut slice[..len/2], level + 1);
            MergeSorter::merge_sort(&mut slice[len/2..], level + 1);

            let left = slice[..len/2].to_vec();
            let right = slice[len/2..].to_vec();
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;
            while k < len {
                /*for _ in 1..level {
                    print!(" ");
                }
                println!("i={} j={} k={}", i, j, k);*/
                if i > left.len() - 1 {
                    slice[k] = right[j];
                    j += 1;
                } else if j > right.len() - 1 {
                    slice[k] = left[i];
                    i += 1;
                } else if left[i] < right[j] {
                    slice[k] = left[i];
                    i+=1;
                } else {
                    slice[k] = right[j];
                    j += 1;
                }
                k+=1;
            }
        }

        /*for _ in 1..level {
            print!(" ");
        }
        println!("merged: ({:?})", slice);*/
    }
}

impl Sorter for MergeSorter {
    fn sort(&mut self, vec: &mut Vec<u64>) {
        MergeSorter::merge_sort(&mut vec[..], 0);
    }
}
