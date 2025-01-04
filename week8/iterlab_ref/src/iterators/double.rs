use super::interleave::Interleave;

/// Creates an iterator that doubles each element of an input iterator.
///
/// This iterator yields each element of the original iterator twice in succession.
/// Is that similar to another thing that you may have implemented in this homework?
///
/// Note that you are allowed to important anything from this crate into this file with `use`.
#[derive(Clone)]
pub struct Double<I> {
    interleave: Interleave<I, I>,
}

impl<I> Double<I>
where
    I: Iterator + Clone,
{
    /// Creates a new `Double` iterator from the given iterator.
    pub fn new(iter: I) -> Self {
        Self {
            interleave: Interleave::new(iter.clone(), iter),
        }
    }
}

/// Implement the `Iterator` trait for `Double`!
impl<I: Iterator + Clone> Iterator for Double<I> {
    /// Output the same type as the input.
    type Item = <I as Iterator>::Item;

    /// Advances the iterator and returns the next doubled element.
    ///
    /// This delegates to the internal `Interleave` iterator to yield each element twice.
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        self.interleave.next()
    }
}
