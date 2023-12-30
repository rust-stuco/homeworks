use crate::multiset::MultiSet;
use std::collections::{BTreeSet, HashMap};
use std::hash::Hash;

/// Trait Alias for all types that implement these traits
trait Value: Eq + Hash {}

pub enum InnerValues<V: Value> {
    SmallSet(Vec<V>),
    LargeSet(MultiSet<V>),
}

impl<V> InnerValues<V>
where
    V: Value,
{
    pub fn contains(&self, key: &V) -> bool {
        match self {
            Self::SmallSet(v) => v.contains(key),
            Self::LargeSet(ms) => ms.contains(key),
        }
    }
}

pub struct MultiMap<K, V>
where
    K: Value,
    V: Value,
{
    inner: HashMap<K, MultiSet<V>>,
}

impl<K, V> MultiMap<K, V>
where
    K: Value,
    V: Value,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        todo!()
    }

    pub fn remove_value(&mut self, key: &K, value: &V) -> bool {
        todo!()
    }

    pub fn remove_key(&mut self, key: &K) -> Option<InnerValues<V>> {
        todo!()
    }

    pub fn get_values(&self, key: &K) -> Option<&InnerValues<V>> {
        todo!()
    }

    pub fn get_values_mut(&mut self, key: &K) -> Option<&mut InnerValues<V>> {
        todo!()
    }
}
