use bitvec::prelude as bv;
use std::marker::PhantomData;

/// An approximate-membership query / probabilistic data structure that supports point lookups.
pub struct BloomFilter<T> {
    /// The inner bitvector / bitset that keeps track of our hashed values.
    ///
    /// Note that you are allowed to implement your own bitvector if you would prefer!
    bitvector: bv::BitVec,

    /// A type marker used to express that this `BloomFilter` is generic over a single type rather
    /// than allowing it to store multiple different types.
    ///
    /// Note that we could have gone with the approach of making only hashing generic, but for
    /// simplicity we will only allow the `BloomFilter` to track elements of a single type.
    phantom: PhantomData<T>,
}

impl BloomFilter {}
