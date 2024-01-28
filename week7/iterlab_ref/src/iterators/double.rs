use super::interleave::Interleave;

pub struct Double<I> {
    interleave: Interleave<I, I>,
}

impl<I> Double<I>
where
    I: Iterator + Clone,
{
    pub fn new(iter: I) -> Self {
        Self {
            interleave: Interleave::new(iter.clone(), iter),
        }
    }
}

impl<I: Iterator + Clone> Iterator for Double<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        self.interleave.next()
    }
}
