// TODO docs
pub struct Fibonacci {
    curr: usize,
    next: usize,
}

// TODO docs
impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

// TODO docs
impl Default for Fibonacci {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

// TODO docs
impl Fibonacci {
    pub fn new(start: usize, next: usize) -> Self {
        Fibonacci { curr: start, next }
    }
}
