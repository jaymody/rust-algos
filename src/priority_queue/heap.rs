use std::cmp::min_by;

use super::PriorityQueue;

pub struct PriorityQueueBinaryHeap<T: Ord, const CAPACITY: usize> {
    arr: [Option<T>; CAPACITY],
    size: usize,
}

impl<T: Ord, const CAPACITY: usize> PriorityQueueBinaryHeap<T, CAPACITY> {
    const INIT: Option<T> = None;
    pub fn new() -> Self {
        PriorityQueueBinaryHeap {
            arr: [Self::INIT; CAPACITY],
            size: 0,
        }
    }

    fn sink(&mut self, i: usize) {
        if i < self.size {
            let c = min_by(2 * i, 2 * i + 1, |x, y| self.arr[*x].cmp(&self.arr[*y]));
            if self.arr[c].is_some() && self.arr[i] > self.arr[c] {
                self.arr.swap(i, c);
                self.sink(c);
            }
        }
    }

    fn swim(&mut self, i: usize) {
        if i > 0 {
            let p = i / 2;
            if self.arr[i] < self.arr[p] {
                self.arr.swap(i, p);
                self.swim(p);
            }
        }
    }
}

impl<T: Ord, const CAPACITY: usize> PriorityQueue<T> for PriorityQueueBinaryHeap<T, CAPACITY> {
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("queue full".to_string());
        }
        self.arr[self.size] = Some(item);
        self.swim(self.size);
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let min = self.arr[0].take();
        self.arr.swap(0, self.size - 1);
        self.size -= 1;
        self.sink(0);
        min
    }

    fn peek(&self) -> Option<&T> {
        self.arr[0].as_ref()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
