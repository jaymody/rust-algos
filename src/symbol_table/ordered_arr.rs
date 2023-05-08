/*
Keys and values are stored in fixed length arrays, sorted in the ordering of
the keys. As such, getting an item is an O(log n) operation.

Putting and popping an item are at worst O(n), and at best O(log n). We first
find the index we need to insert the key/value pair (which we can find via
binary search), and then we need to either update it and we are done, or we need
to insert a new entry in which case we'll need to shift the array (which if the
position to insert is at the start, is an O(n) operation, if at the end, is
an O(1) operation).
*/

use crate::{
    search::{binary_search, binary_search_insert_index},
    utils::{insert_and_shift, pop_and_shift},
};

use super::{KeyT, SymbolTable};

pub struct OrderedArrST<K: KeyT, V, const CAPACITY: usize> {
    keys: [Option<K>; CAPACITY],
    vals: [Option<V>; CAPACITY],
    size: usize,
}

impl<K: KeyT, V, const CAPACITY: usize> OrderedArrST<K, V, CAPACITY> {
    const K_INIT: Option<K> = None;
    const V_INIT: Option<V> = None;

    pub fn new() -> Self {
        OrderedArrST {
            keys: [Self::K_INIT; CAPACITY],
            vals: [Self::V_INIT; CAPACITY],
            size: 0,
        }
    }
}

impl<K: KeyT, V, const CAPACITY: usize> SymbolTable<K, V> for OrderedArrST<K, V, CAPACITY> {
    fn put(&mut self, key: K, val: V) -> Result<(), String> {
        let key = Some(key);
        let val = Some(val);
        let i = binary_search_insert_index(&self.keys[..self.size], &key);
        if self.keys[i] == key {
            // update the entry
            self.vals[i] = val;
        } else {
            // insert a new entry
            if self.size >= CAPACITY {
                return Err("capacity full".to_string());
            }
            self.keys[self.size] = insert_and_shift(&mut self.keys[..self.size], key, i);
            self.vals[self.size] = insert_and_shift(&mut self.vals[..self.size], val, i);
            self.size += 1;
        }
        Ok(())
    }

    fn get(&self, key: K) -> Option<&V> {
        let i = binary_search(&self.keys[..self.size], &Some(key))?;
        self.vals[i].as_ref()
    }

    fn pop(&mut self, key: K) -> Option<V> {
        let i = binary_search(&self.keys[..self.size], &Some(key))?;
        self.size -= 1;
        pop_and_shift(&mut self.keys[..self.size + 1], None, i);
        pop_and_shift(&mut self.vals[..self.size + 1], None, i)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

pub struct IntoIter<'a, K: KeyT, V, const CAPACITY: usize> {
    st: &'a OrderedArrST<K, V, CAPACITY>,
    i: usize,
}

impl<'a, K: KeyT, V, const CAPACITY: usize> Iterator for IntoIter<'a, K, V, CAPACITY> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.st.keys[self.i].as_ref()
    }
}

impl<'a, K: KeyT, V, const CAPACITY: usize> IntoIterator for &'a OrderedArrST<K, V, CAPACITY> {
    type Item = &'a K;
    type IntoIter = IntoIter<'a, K, V, CAPACITY>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { st: self, i: 0 }
    }
}
