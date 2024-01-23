pub struct FilterMap<I, F> {
    iter: I,
    f: F,
}

impl<B, I: Iterator, F> Iterator for FilterMap<I, F>
where
    F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            match (self.f)(item) {
                None => continue,
                x => return x,
            }
        }
        None
    }
}
