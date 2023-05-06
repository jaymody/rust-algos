/*
The approach used here is to keep a fixed length arr with items that unordered
and keep a value that tracks the current number of items.

Pushing to the pq requires O(1) time. We just add the item to the end of the
array.

Peeking and popping require O(n) time. For peeking, we linearly search the array
for the smallest entry. For popping, we do the same but once we find the entry,
we exchange it with the last entry and then remove the last entry (which since
we swapped, is the smallest item).
*/

use super::PriorityQueue;

pub struct PriorityQueueUnorderedArr<T: Ord, const CAPACITY: usize> {
    arr: [Option<T>; CAPACITY],
    size: usize,
}

impl<T: Ord, const CAPACITY: usize> PriorityQueueUnorderedArr<T, CAPACITY> {
    const INIT: Option<T> = None;

    pub fn new() -> Self {
        PriorityQueueUnorderedArr {
            arr: [Self::INIT; CAPACITY],
            size: 0,
        }
    }

    fn get_min_i(&self) -> Option<usize> {
        if self.size == 0 {
            return None;
        }
        let mut min_i = 0;
        for i in 0..self.size {
            if self.arr[i] < self.arr[min_i] {
                min_i = i;
            }
        }
        Some(min_i)
    }
}

impl<T: Ord, const CAPACITY: usize> PriorityQueue<T> for PriorityQueueUnorderedArr<T, CAPACITY> {
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("min pq full".to_string());
        }
        self.arr[self.size] = Some(item);
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        let min_i = self.get_min_i()?;
        self.arr.swap(min_i, self.size - 1);
        self.size -= 1;
        self.arr[self.size].take()
    }

    fn peek(&self) -> Option<&T> {
        let min_i = self.get_min_i()?;
        self.arr[min_i].as_ref()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
