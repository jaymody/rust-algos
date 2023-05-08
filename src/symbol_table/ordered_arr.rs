use crate::{
    search::{binary_search, binary_search_insert_index},
    utils::{insert_and_shift, pop_and_shift},
};

use super::{KeyT, SymbolTable};

/// A symbol table where keys and values are stored in fixed length arrays,
/// sorted in the ordering of the keys.
///
/// Since the entries are sorted, we leverage binary search to find an entry
/// in O(log n) time.
///
/// As such, the get operation is runs in  O(log n) time.
///
/// Insertion and deletion remain O(n) however. Even though you can find
/// the index of insertion/deletion in O(log n) time, you need to shift all the
/// entries to the right for insertion and to the left for deletion, which is
/// an O(n) operation.
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
    /// Add (or update) the key-value pair.
    ///
    /// ### Implementation
    /// We use binary search and either:
    ///
    ///   1) Update: The key already exists and binary search finds it in
    ///      O(log n) time and we simply update the value.
    ///   2) Insert: The key does not exist and binary search finds at which
    ///      position we need to insert the new entry to keep the arrays sorted.
    ///      While finding the insertion index is an O(log n) operation, we need
    ///      to shift all the position to the right by one to make space for the
    ///      insertion, which is an O(n) operation.
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

    /// Get a reference to the value for the associated key (None if the key
    /// does not exist).
    ///
    /// ### Implementation
    /// We perform binary search on the array. If there is a hit, we return it,
    /// else the key does not exist and we return None.
    fn get(&self, key: K) -> Option<&V> {
        let i = binary_search(&self.keys[..self.size], &Some(key))?;
        self.vals[i].as_ref()
    }

    /// Remove the entry that matches the key, and return it's value (return
    /// None if the key does not exist).
    ///
    /// ### Implementation
    /// We perform binary search and either:
    ///
    ///   1) The key does not exist, and we return None.
    ///   2) The key does exist and we find it's index. In this case, we need
    ///      to delete the entry from the array and shift the values to the left
    ///      by 1 to fill the vacant position. While finding the index is an
    ///      O(log n) operation, shifting the values is an O(n) operation.
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
        if self.i >= self.st.size() {
            None
        } else {
            self.i += 1;
            self.st.keys[self.i - 1].as_ref()
        }
    }
}

impl<'a, K: KeyT, V, const CAPACITY: usize> IntoIterator for &'a OrderedArrST<K, V, CAPACITY> {
    type Item = &'a K;
    type IntoIter = IntoIter<'a, K, V, CAPACITY>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { st: self, i: 0 }
    }
}
