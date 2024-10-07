// TODO remove
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

mod bitvector;
mod bloomfilter;

use bitvector::BitVector;
pub use bloomfilter::BloomFilter;
