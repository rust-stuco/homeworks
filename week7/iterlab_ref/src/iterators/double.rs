pub struct Double<I> {
    _phantom: std::marker::PhantomData<I>,
}

impl<I> Double<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        todo!()
    }
}


impl<I: Iterator + Clone> Iterator for Double<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        todo!()
    }
}
