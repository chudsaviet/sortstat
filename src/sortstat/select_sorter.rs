use crate::sortstat::sorter_trait::Sorter;

pub struct SelectSorter {}

impl Sorter for SelectSorter {
    fn sort(&mut self, vec: &mut Vec<u64>) {
        for beginning in 0..vec.len() {
            for i in beginning..vec.len() {
                if vec[i] < vec[beginning] {
                    let buf = vec[i];
                    vec[i] = vec[beginning];
                    vec[beginning] = buf;
                }
            }
        }
    }
}
