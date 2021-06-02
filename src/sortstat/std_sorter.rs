use crate::sortstat::sorter_trait::Sorter;

pub struct StdSorter {}

impl Sorter for StdSorter {
    fn sort(&mut self, vec: &mut Vec<u64>) {
        vec.sort();
    }
}
