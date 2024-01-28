

pub struct Extend<I> {
    original: I,
    iter: I,
}

impl<I: Clone> Extend<I> {
    pub fn new(iter: I) -> Self {
        Self { original: iter.clone(), iter }
    }
}

impl<I: Clone + Iterator> Iterator for Extend<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        todo!()
    }
}
