use crate::multiset::MultiSet;
use std::collections::HashMap;
use std::hash::Hash;

/// Trait Alias for all types that implement these traits
pub trait Value: Eq + Hash {}

#[non_exhaustive]
pub enum Values<V: Value> {
    SmallSet(Vec<V>),
    LargeSet(MultiSet<V>),
}

impl<V: Value> Values<V> {
    pub fn contains(&self, key: &V) -> bool {
        match self {
            Self::SmallSet(v) => v.contains(key),
            Self::LargeSet(ms) => ms.contains(key),
        }
    }

    fn transform_large(&mut self) {
        match self {
            Self::LargeSet(_) => (),
            Self::SmallSet(vs) => {
                let mut ms = MultiSet::new();
                while let Some(element) = vs.pop() {
                    ms.insert(element);
                }
                *self = Self::LargeSet(ms);
            }
        }
    }
}

pub struct MultiMap<K: Value, V: Value> {
    inner: HashMap<K, Values<V>>,
}

impl<K: Value, V: Value> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let values = self.inner.entry(key).or_insert(Values::SmallSet(vec![]));

        if let Values::SmallSet(vs) = values {
            if vs.len() >= 98 {
                values.transform_large();
            }
        }

        match values {
            Values::SmallSet(vs) => vs.push(value),
            Values::LargeSet(ms) => ms.insert(value),
        }
    }

    pub fn remove_value(&mut self, key: &K, value: &V) -> bool {
        match self.inner.get_mut(key) {
            None => false,
            Some(values) => match values {
                Values::SmallSet(vs) => match vs.iter().position(|x| x == value) {
                    None => false,
                    Some(i) => {
                        vs.remove(i);
                        true
                    }
                },
                Values::LargeSet(ms) => ms.remove(value),
            },
        }
    }

    pub fn remove_key(&mut self, key: &K) -> Option<Values<V>> {
        self.inner.remove(key)
    }

    pub fn get_values(&self, key: &K) -> Option<&Values<V>> {
        self.inner.get(key)
    }

    pub fn get_values_mut(&mut self, key: &K) -> Option<&mut Values<V>> {
        self.inner.get_mut(key)
    }
}

impl<K: Value, V: Value> Default for MultiMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
