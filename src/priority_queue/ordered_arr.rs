use crate::{search::binary_search_insert_index_rev, utils::insert_and_shift};

use super::PriorityQueue;

/// An ordered fixed length array implementation for a priority queue.
///
/// The approach here is to keep a fixed length arr with items in reverse
/// sorted order (descending sorted). Sorting enables O(1) popping/peeking, at
/// the cost of O(n) insertions.
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
    /// Push an item to the queue.
    ///
    /// ### Implementation
    /// Pushing to the queue corresponds to finding the position of insertion
    /// that keeps the array sorted (which we can do in O(log n) time via
    /// binary search) and then shifting the values to the right of it by 1 to
    /// make space for it (which requires O(n) time).
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

    /// Pop an item from the queue (return None if the queue is empty).
    ///
    /// ### Implementation
    /// Since the array is reverse sorted, we simply remove and return the last
    /// entry in our array, which is an O(1) operation.
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.arr[self.size].take()
    }

    /// Peek at the next item in the queue (return None if the queue is empty).
    ///
    /// ### Implementation
    /// Since the array is reverse sorted, we simply return a reference to the
    /// last entry in the array.
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
