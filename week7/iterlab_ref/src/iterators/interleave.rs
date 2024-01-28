#[derive(Clone)]
pub struct Interleave<I, J> {
    first_iter: I,
    second_iter: J,
    switch: bool,
}

impl<I, J> Interleave<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    pub fn new(first_iter: I, second_iter: J) -> Self {
        Self {
            first_iter,
            second_iter,
            switch: true,
        }
    }
}

impl<I, J> Iterator for Interleave<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        self.switch = !self.switch;

        if !self.switch {
            match self.first_iter.next() {
                None => self.second_iter.next(),
                r => r,
            }
        } else {
            match self.second_iter.next() {
                None => self.first_iter.next(),
                r => r,
            }
        }
    }
}
