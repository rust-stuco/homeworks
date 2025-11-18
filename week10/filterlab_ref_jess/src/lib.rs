#![doc = include_str!("../README.md")]

mod bitvector_vanilla;
// mod bitvector_x86_avx;
mod bloomfilter;
mod doublehasher;
mod doublehasher_x86_avx;

pub use bloomfilter::BloomFilter;
