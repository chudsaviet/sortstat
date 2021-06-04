use crate::sortstat::sorter_trait::Sorter;

pub struct QuickSorter {}

impl QuickSorter {
    fn quick_sort(slice: &mut [u64], level: u32) {
        let len = slice.len();
        if len > 1 {
            let mut pivot = len/2;
            loop {
                let mut bigger_found = false;
                let mut bigger = 0;
                for i in 0..pivot {
                    if slice[i] > slice[pivot] {
                        bigger_found = true;
                        bigger = i;
                        break;
                    }
                }

                let mut lesser_found = false;
                let mut lesser = 0;
                for i in pivot+1..len {
                    if slice[i] < slice[pivot] {
                        lesser_found = true;
                        lesser = i;
                        break;
                    }
                }

                if bigger_found && lesser_found {
                    let buf = slice[bigger];
                    slice[bigger] = slice[lesser];
                    slice[lesser] = buf;
                } else if bigger_found {
                    let buf = slice[pivot];
                    slice[pivot] = slice[bigger];
                    slice[bigger] = buf;
                    pivot -= 1;
                } else if lesser_found {
                    let buf = slice[pivot];
                    slice[pivot] = slice[lesser];
                    slice[lesser] = buf;
                    pivot += 1;
                } else {
                    break;
                }
            }
            QuickSorter::quick_sort(&mut slice[..pivot], level + 1);
            QuickSorter::quick_sort(&mut slice[pivot+1..], level + 1);
        }
    }
}

impl Sorter for QuickSorter {
    fn sort(&mut self, vec: &mut Vec<u64>) {
        QuickSorter::quick_sort(&mut vec[..], 0)
    }
}
