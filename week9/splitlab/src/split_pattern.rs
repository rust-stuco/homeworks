/// A struct that represents split operations on a string.
///
/// TODO(student): You will need to change the `SplitPattern` struct to use lifetime parameter(s).
pub struct SplitPattern<P> {
    /// The remainder of the string that has not yet been split.
    ///
    /// Before the iterator has yielded any substrings, this is the entire string.
    /// After each call to `next`, this is the part of the string that has not yet been split.
    ///
    /// TODO(student): Replace the `'static` lifetime with something else!
    remainder: Option<&'static str>,

    /// The generic delimiter pattern used to split the haystack string.
    delimiter: P,
}

impl<P> SplitPattern<P> {
    /// Creates a new `Split` instance with the given haystack and delimiter.
    ///
    /// TODO(student): Replace the `'static` lifetime with something else!
    pub fn new(haystack: &'static str, delimiter: P) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<P> Iterator for SplitPattern<P>
where
    P: Pattern,
{
    /// This iterator yields substrings of the original `haystack` string, split by some delimiter
    /// pattern.
    ///
    /// TODO(student): Replace the `'static` lifetime with something else!
    type Item = &'static str;

    /// Returns the next substring of the original `haystack` string, split by some delimiter
    /// pattern.
    ///
    /// Panics if the delimiter is empty (length 0).
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement me (make sure to fix the lifetimes!)")
    }
}

/// An interface for a type that can find itself in a string.
pub trait Pattern {
    /// Finds the next occurrence of the pattern in the given string.
    ///
    /// Returns `Some((start, end))` if the pattern is found, where `start` is starting index of the
    /// pattern and `end` is the index of the end of the pattern in the string.
    /// Returns `None` if the pattern is not found.
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Pattern for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Pattern for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // Remember that characters are not always one byte long! Make sure to use `len_utf8`.
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

impl<F> Pattern for F
where
    F: Fn(char) -> bool,
{
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| self(*c))
            .map(|(start, c)| (start, start + c.len_utf8()))
    }
}

impl Pattern for [char; 1] {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        self[0].find_next(s)
    }
}

impl Pattern for &[char] {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        if self.is_empty() {
            return Some((0, 0));
        }

        s.char_indices()
            .find(|(_, c)| self.contains(c))
            .map(|(start, c)| (start, start + c.len_utf8()))
    }
}
