pub struct Cycle<I> {
    original: I,
    iter: I,
}

impl<I: Clone> Cycle<I> {
    pub fn new(iter: I) -> Self {
        Self {
            original: iter.clone(),
            iter,
        }
    }
}

impl<I: Clone + Iterator> Iterator for Cycle<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        match self.iter.next() {
            None => {
                self.iter = self.original.clone();
                self.iter.next()
            }
            item => item,
        }
    }
}
