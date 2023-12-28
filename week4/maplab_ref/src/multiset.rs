use std::collections::HashMap;
use std::hash::Hash;

/// A hash-based multiset.
pub struct MultiSet<K: Eq + Hash> {
    inner: HashMap<K, usize>,
    size: usize,
}

#[allow(clippy::new_without_default)]
impl<K> MultiSet<K>
where
    K: Eq + Hash,
{
    /// Creates a new empty `MultiSet`.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            size: 0,
        }
    }

    /// Counts all the elements, including each duplicate.
    ///
    /// # Examples
    ///
    /// A new empty `MultiSet` with 0 total elements:
    ///
    /// ```
    /// use maplab_ref::multiset::MultiSet;
    ///
    /// let multiset: MultiSet<char> = MultiSet::new();
    /// assert_eq!(0, multiset.len());
    /// ```
    ///
    /// A `MultiSet` from `vec![1,1,2]` has 3 total elements:
    ///
    /// TODO example
    pub fn len(&self) -> usize {
        self.size
    }

    /// TODO documentation and example
    pub fn contains(&self, value: &K) -> bool {
        self.inner.contains_key(value)
    }

    /// Inserts an element.
    ///
    /// # Examples
    ///
    /// Insert `5` into a new `MultiSet`:
    ///
    /// ```
    /// use maplab_ref::multiset::MultiSet;
    ///
    /// let mut multiset: MultiSet<i32> = MultiSet::new();
    /// assert_eq!(0, multiset.count(&5));
    /// multiset.insert(5);
    /// assert_eq!(1, multiset.count(&5));
    /// ```
    pub fn insert(&mut self, value: K) {
        self.size += 1;
        *self.inner.entry(value).or_insert(0) += 1;
    }

    /// Remove an element. Removal of a nonexistent element
    /// has no effect.
    ///
    /// # Examples
    ///
    /// Remove `5` from a new `MultiSet`:
    ///
    /// ```
    /// # use maplab_ref::multiset::MultiSet;
    /// #
    /// let mut multiset: MultiSet<i32> = MultiSet::new();
    /// multiset.insert(5);
    /// assert_eq!(1, multiset.count(&5));
    /// assert!(multiset.remove(&5));
    /// assert_eq!(0, multiset.count(&5));
    /// assert!(!multiset.remove(&5));
    /// ```
    pub fn remove(&mut self, value: &K) -> bool {
        match self.inner.get_mut(value) {
            None => false,
            Some(count) => {
                debug_assert!(*count > 0);
                if *count == 1 {
                    self.inner.remove(value);
                } else {
                    *count -= 1;
                }
                true
            }
        }
    }

    /// Counts the occurrences of `val`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use maplab_ref::multiset::MultiSet;
    /// #
    /// let mut multiset: MultiSet<u8> = MultiSet::new();
    /// multiset.insert(0);
    /// multiset.insert(0);
    /// multiset.insert(1);
    /// multiset.insert(0);
    /// assert_eq!(3, multiset.count(&0));
    /// assert_eq!(1, multiset.count(&1));
    /// ```
    pub fn count(&self, value: &K) -> usize {
        self.inner.get(value).map_or(0, |x| *x)
    }
}
