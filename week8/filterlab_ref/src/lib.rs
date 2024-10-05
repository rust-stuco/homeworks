use bitvec::prelude as bv;
use std::hash::Hash;
use std::marker::PhantomData;

/// An approximate-membership query / probabilistic data structure that supports point lookups.
pub struct BloomFilter<T> {
    /// The inner bitvector / bitset that keeps track of our hashed values.
    ///
    /// Note that you are allowed to implement your own bitvector if you would prefer!
    bitvector: bv::BitVec,

    /// Several statistics related to the bloom filter data structure.
    stats: BloomFilterStats,

    /// A type marker used to express that this `BloomFilter` is generic over a single type rather
    /// than allowing it to store multiple different types.
    ///
    /// Note that we could have gone with the approach of making only hashing generic, but for
    /// simplicity we will only allow the `BloomFilter` to track elements of a single type.
    phantom: PhantomData<T>,
}

struct BloomFilterStats {
    /// The total number of bits in the bloom filter's bitvector.
    num_bits: usize,

    /// The maximum number of elements that this bloom filter is allowed to store to maintain its
    /// theoretical false positive rate.
    max_elements: usize,

    /// The approximate current number of elements that this bloom filter is storing.
    curr_elements: usize,

    /// The number of hash functions per element.
    num_hashes: usize,
}

impl<T: Hash> BloomFilter<T> {
    /// Creates a new `BloomFilter` given the maximum number of elements that will be inserted into
    /// the filter and a bound on the false positive rate.
    pub fn new_with_fpr(max_elements: usize, fpr: f64) -> Self {
        todo!()
    }

    /// Creates a new `BloomFilter` given the maximum number of elements that will be inserted into
    /// the filter and a bound on the size of the `BloomFilter`'s bitvector
    pub fn new_with_size(max_elements: usize, num_bits: usize) -> Self {
        todo!()
    }

    /// Inserts an element into the bloom filter.
    ///
    /// Returns `true` if the element might have been in the bloom filter previously before
    /// insertion.
    pub fn insert(&mut self, elem: &T) -> bool {
        todo!()
    }

    /// Checks if an element might have been previously inserted into the bloom filter.
    pub fn contains(&mut self, elem: &T) -> bool {
        todo!()
    }
}
