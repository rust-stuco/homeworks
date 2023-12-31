use std::collections::HashMap;
use std::hash::Hash;

pub struct MultiMap<K: Hash + Eq, V: Eq> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: Hash + Eq, V: Eq> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.entry(key).or_default().push(value);
    }

    pub fn remove_value(&mut self, key: &K, value: &V) -> bool {
        match self.inner.get_mut(key) {
            None => false,
            Some(values) => match values.iter().position(|x| x == value) {
                None => false,
                Some(i) => {
                    values.remove(i);
                    true
                }
            },
        }
    }

    pub fn remove_key(&mut self, key: &K) -> Option<Vec<V>> {
        self.inner.remove(key)
    }

    pub fn get_values(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
    }

    pub fn get_values_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        self.inner.get_mut(key)
    }
}

impl<K: Hash + Eq, V: Eq> Default for MultiMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
