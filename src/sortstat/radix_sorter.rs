use crate::sortstat::sorter_trait::Sorter;


pub struct RadixSorter {}

impl RadixSorter {
    fn get_digit(number: u64, digit: u8) -> u8 {
        (
            (number as u128%10u128.pow((digit+1) as u32) - number as u128%10u128.pow(digit as u32))
            /10u128.pow(digit as u32)
        ) as u8
    }

    fn radix_sort(slice: &mut [u64]) {
        let len = slice.len();
        // Best guess is 2/20 of the overall size.
        let bucket_capacity = len/10;
        // Yes, its the best way to initialize an array of vectors.
        let mut buckets: [&mut Vec<u64>; 20] = [
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity),
            &mut Vec::with_capacity(bucket_capacity)
        ];

        for digit in 0..20u8 {
            for element in &mut *slice {
                let digit_value = RadixSorter::get_digit(*element, digit);
                buckets[digit_value as usize].push(*element);
            }

            let mut i = 0;
            for bucket in buckets.iter_mut() {
                for element in bucket.iter() {
                    slice[i] = *element;
                    i += 1;
                }
                bucket.clear();
            }
        }
    }
}

impl Sorter for RadixSorter {
    fn sort(&mut self, vec: &mut Vec<u64>) {
        RadixSorter::radix_sort(&mut vec[..]);
    }
}
