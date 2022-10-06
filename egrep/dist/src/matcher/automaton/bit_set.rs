// Should be understood as `u8::MAX.div_ceil(u64::BITS)`
const SIZE: usize = 1 + ((u8::MAX as u32) / (u64::BITS)) as usize;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct BitSet([u64; SIZE]);

impl BitSet {
    pub(super) fn new() -> Self {
        Self([0; SIZE])
    }

    pub(super) fn with_all_true() -> Self {
        Self([!0; SIZE])
    }

    pub(super) fn singelton(i: u8) -> Self {
        let mut bitset = Self::new();
        bitset.insert(i);
        bitset
    }

    pub(super) fn insert(&mut self, i: u8) {
        let bucket = i / u64::BITS as u8;
        let bit = i % u64::BITS as u8;
        self.0[bucket as usize] |= 1 << bit;
    }

    pub(super) fn remove(&mut self, i: u8) {
        let bucket = i / u64::BITS as u8;
        let bit = i % u64::BITS as u8;
        self.0[bucket as usize] &= !(1 << bit);
    }

    pub(super) fn get(&self, i: u8) -> bool {
        let bucket = i / u64::BITS as u8;
        let bit = i % u64::BITS as u8;
        self.0[bucket as usize] >> bit & 1 == 1
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_set() {
        let bs = BitSet::new();
        for i in 0..=u8::MAX {
            assert!(!bs.get(i));
        }
    }

    #[test]
    fn all_true() {
        let bs = BitSet::with_all_true();
        for i in 0..=u8::MAX {
            assert!(bs.get(i));
        }
    }

    #[test]
    fn singeltons() {
        for i in 0..=u8::MAX {
            let bs = BitSet::singelton(i);
            for j in 0..=u8::MAX {
                assert_eq!(bs.get(j), i == j);
            }
        }
    }

    #[test]
    fn sets_and_gets() {
        let mut bs = BitSet::new();
        assert!(!bs.get(65));
        assert!(!bs.get(37));

        bs.insert(37);
        assert!(bs.get(37));
        assert!(!bs.get(65));

        bs.insert(65);
        assert!(bs.get(37));
        assert!(bs.get(65));

        bs.insert(139);
        assert!(bs.get(139));
    }

    #[test]
    fn removals() {
        let mut bs = BitSet::new();
        assert!(!bs.get(65));
        assert!(!bs.get(37));

        bs.insert(37);
        assert!(bs.get(37));
        assert!(!bs.get(65));

        bs.insert(65);
        assert!(bs.get(37));
        assert!(bs.get(65));

        bs.remove(65);
        assert!(bs.get(37));
        assert!(!bs.get(65));

        bs.insert(139);
        assert!(bs.get(139));

        bs.remove(37);
        assert!(!bs.get(37));

        bs.remove(37);
    }
}
