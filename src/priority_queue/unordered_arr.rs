use super::PriorityQueue;

/// An unordered fixed length array implementation for a priority queue.
///
/// The approach here is to keep a fixed length arr with items in unordered.
/// Keeping the items unordered enables us to insert in O(1) time, at the cost
/// of O(n) peeks/pops.
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

    /// Get the index of the minimum entry (None if the queue is empty).
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
    /// Push an item to the queue.
    ///
    /// ### Implementation
    /// Since the array is unordered, we can just add the entry to the end
    /// of the array in O(1) time.
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("capacity full".to_string());
        }
        self.arr[self.size] = Some(item);
        self.size += 1;
        Ok(())
    }

    /// Pop an item from the queue (return None if the queue is empty).
    ///
    /// ### Implementation
    /// Since the array is unordered, we need to perform a linear search to
    /// find the minimum entry. Instead of removing and returning the entry and
    /// then shifting the rest of the array to the left to fill the vacant spot,
    /// we can instead swap the minimum entry with the last entry so we needn't
    /// shift the array.
    fn pop(&mut self) -> Option<T> {
        let min_i = self.get_min_i()?;
        self.arr.swap(min_i, self.size - 1);
        self.size -= 1;
        self.arr[self.size].take()
    }

    /// Peek at the next item in the queue (return None if the queue is empty).
    ///
    /// ### Implementation
    /// Since the array is unordered, we need to perform a linear search to
    /// find the minimum entry.
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
