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
        let l = 2 * i;
        let r = 2 * i + 1;

        if l < self.size {
            let min_child_i = if r < self.size && self.arr[r] < self.arr[l] {
                r
            } else {
                l
            };

            if self.arr[i] > self.arr[min_child_i] {
                self.arr.swap(i, min_child_i);
                self.sink(min_child_i);
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
            return Err("capacity full".to_string());
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
