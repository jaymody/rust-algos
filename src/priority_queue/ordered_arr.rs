/*
The approach used here is to keep a fixed length arr with items in reverse
sorted order (descending sorted) and a value that tracks the current number
of items.

Pushing to the queue requires O(n) time. We first find the position the item
belongs (which we can do using binary search since the array is sorted) which is
O(log n) time, but then we need to shift the rest of the items, which is in
the worst case an O(n) operation (i.e. if the item needs to be inserted at the
start of the array).

Peeking and popping require O(1) time, since the array is sorted, we simply
retrieve the last entry.
*/

use crate::{search::binary_search_insert_index_rev, utils::insert_and_shift};

use super::PriorityQueue;

pub struct PriorityQueueOrderedArr<T: Ord, const CAPACITY: usize> {
    pub arr: [Option<T>; CAPACITY],
    size: usize,
}

impl<T: Ord, const CAPACITY: usize> PriorityQueueOrderedArr<T, CAPACITY> {
    const INIT: Option<T> = None;

    pub fn new() -> Self {
        PriorityQueueOrderedArr {
            arr: [Self::INIT; CAPACITY],
            size: 0,
        }
    }
}

impl<T: Ord, const CAPACITY: usize> PriorityQueue<T> for PriorityQueueOrderedArr<T, CAPACITY> {
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("capacity full".to_string());
        }

        let item = Some(item);
        let i = binary_search_insert_index_rev(&self.arr[..self.size], &item);
        self.arr[self.size] = insert_and_shift(&mut self.arr[..self.size], item, i);
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.arr[self.size].take()
    }

    fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.arr[self.size - 1].as_ref()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
