use crate::sortstat::sorter_trait::Sorter;

pub struct StdSorterUnstable {}

impl Sorter for StdSorterUnstable {
    fn sort(&mut self, vec: &mut Vec<u64>) {
        vec.sort_unstable();
    }
}
