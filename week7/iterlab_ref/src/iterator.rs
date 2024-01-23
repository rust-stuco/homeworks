use crate::filter_map::FilterMap;

pub trait StuCoIterator: Iterator<Item = Self::StucoItem> {
    type StucoItem;

    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::StucoItem) -> Option<B>;
}
