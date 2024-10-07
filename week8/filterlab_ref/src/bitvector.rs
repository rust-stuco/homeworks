/// A simple bitvector backed by a [`Vec<u8>`].
#[derive(Debug, Clone, Default)]
pub struct BitVector {
    inner: Vec<u8>,
}

impl BitVector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, index: usize, value: bool) {
        todo!()
    }

    pub fn get(&mut self, index: usize) -> bool {
        todo!()
    }
}