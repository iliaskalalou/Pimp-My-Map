use core::hash::Hash;
use std::collections::{hash_map::Entry, HashMap};

// use crate::ui::ui::StaticWidgets;
// use crate::widgets::*;

// use super::maths::Vec2u;

pub type IndexType = u8;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Index(IndexType);

#[derive(Clone, Default)]
pub struct MultiMap<K, V>
where
    K: core::hash::Hash + Eq,
{
    //we use usize to detect if key is an alias
    keys: HashMap<K, (Index, usize)>,
    data: HashMap<Index, V>,
    unique_keys: IndexType,
}

impl<K, V> MultiMap<K, V>
where
    K: core::hash::Hash + Eq,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            keys: HashMap::with_capacity(capacity),
            data: HashMap::with_capacity(capacity),
            unique_keys: IndexType::default(),
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.data.clear();
        self.unique_keys = IndexType::default();
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.keys.contains_key(k)
    }

    pub fn next_index(&self) -> Index {
        Index(self.unique_keys as IndexType)
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.keys.get(k).and_then(|(idx, _)| self.data.get(idx))
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.keys.get(k).and_then(|(idx, _)| self.data.get_mut(idx))
    }

    pub fn keys_get(&self, k: &K) -> Option<&Index> {
        self.keys.get(k).map(|(x, _)| x)
    }

    pub fn data_get(&self, idx: &Index) -> Option<&V> {
        self.data.get(idx)
    }

    pub fn data_get_mut(&mut self, idx: &Index) -> Option<&mut V> {
        self.data.get_mut(idx)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let i = self.next_index();
        self.insert_key(k, i);
        self.data.insert(i, v)
    }

    pub fn insert_key(&mut self, k: K, i: Index) -> Option<Index> {
        self.unique_keys += 1;
        self.keys.insert(k, (i, 1)).map(|(x, _)| x)
    }

    pub fn insert_data(&mut self, k: K, v: V) -> Option<V> {
        self.data
            .insert(*self.keys.get(&k).map(|(x, _)| x).unwrap(), v)
    }

    pub fn unchecked_get(&self, k: &K) -> &V {
        self.get(k).unwrap()
    }

    pub fn unchecked_get_mut(&mut self, k: &K) -> &mut V {
        self.get_mut(k).unwrap()
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        let idx = self.remove_key(k).map(|(x, _)| x)?;
        self.data.remove(&idx)
    }

    pub fn remove_key(&mut self, k: &K) -> Option<(Index, usize)> {
        if self.is_key(k) {
            self.keys.remove(k)
        } else {
            None
        }
    }

    pub fn remove_alias(&mut self, k: &K) {
        if self.is_alias(k) {
            self.keys.remove(k);
        }
    }

    //handles keys and aliases without distinction
    pub fn destroy_key(&mut self, k: &K) -> Option<(Index, usize)> {
        self.keys.remove(k)
    }

    pub fn unique_keys_count(&self) -> IndexType {
        self.unique_keys
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.keys()
    }

    pub fn key_values(&self) -> impl Iterator<Item = (&K, &Index)> {
        self.keys.iter().map(|(x, (xidx, _))| (x, xidx))
    }

    pub fn keys_values_mut(
        &mut self,
    ) -> impl Iterator<Item = (&K, &mut Index)> {
        self.keys.iter_mut().map(|(x, (xidx, _))| (x, xidx))
    }

    pub fn data(&self) -> impl Iterator<Item = (&Index, &V)> {
        self.data.iter()
    }

    pub fn data_mut(&mut self) -> impl Iterator<Item = (&Index, &mut V)> {
        self.data.iter_mut()
    }

    pub fn data_values(&self) -> impl Iterator<Item = &V> {
        self.data.values()
    }

    pub fn data_values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.data.values_mut()
    }

    pub fn keys_len(&self) -> usize {
        self.keys.len()
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn entry(&mut self, k: K) -> Entry<'_, Index, V> {
        self.data.entry(*self.keys.get(&k).map(|(x, _)| x).unwrap())
    }

    pub fn replace_key(&mut self, old: &K, new: K) -> Option<(Index, usize)> {
        if self.is_key(old) {
            let tmp = self.keys.remove(old).unwrap();
            self.keys.insert(new, tmp)
        } else {
            None
        }
    }

    pub fn alias(&mut self, base: &K, alias: K) -> Option<Index> {
        if !self.is_alias(base) {
            let original = self.keys.get_mut(base).unwrap();
            original.1 += 1;
            let idx = original.0;
            self.keys.insert(alias, (idx, 0)).map(|(x, _)| x)
        } else {
            None
        }
    }

    pub fn unalias(&mut self, base: &K) -> Vec<K>
    where
        K: core::hash::Hash + Eq + Copy,
    {
        if !self.is_key(base) {
            return Vec::new();
        }

        let (idx, _) = *self.keys.get(base).unwrap();
        let res: Vec<K> = self
            .keys
            .iter()
            .filter(|(x, (xidx, _))| *xidx == idx && **x != *base)
            .map(|(x, _)| *x)
            .collect();

        self.keys.retain(|x, (xidx, _)| *xidx != idx || *x == *base);

        res
    }

    pub fn is_key(&self, k: &K) -> bool {
        if let Some(tuple) = self.keys.get(k) {
            tuple.1 != 0
        } else {
            false
        }
    }

    pub fn is_alias(&self, k: &K) -> bool {
        if let Some(tuple) = self.keys.get(k) {
            tuple.1 == 0
        } else {
            false
        }
    }

    pub fn is_alias_of(&self, base: &K, other: &K) -> bool {
        let (a, _) = self.keys.get(base).unwrap();
        let (b, _) = self.keys.get(other).unwrap();

        self.is_key(base) && self.is_alias(other) && a == b
    }

    pub fn order(&self) -> usize {
        let mut map: HashMap<Index, bool> = HashMap::new();

        self.data.keys().for_each(|x| {
            map.insert(*x, true);
        });

        map.values().filter(|x| **x).count()
    }
}
