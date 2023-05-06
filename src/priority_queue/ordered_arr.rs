/*
The approach used here is to keep a fixed length arr with items in reverse
sorted order (descending sorted) and a value that tracks the current number
of items.

Pushing to the queue requires O(n) time. We first find the position the item
belongs and then we shift the rest of the items to make space for it.

Peeking and popping require O(1) time, since the array is sorted, we simply
retrieve the last entry.
*/

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
            return Err("queue full".to_string());
        }

        let mut i = 0;
        while i < self.size && self.arr[i].as_ref().unwrap() > &item {
            i += 1;
        }

        let mut prev = Some(item);
        for j in i..=self.size {
            (self.arr[j], prev) = (prev, self.arr[j].take());
        }
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
