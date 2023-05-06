use super::MinHeap;

pub struct MinHeapOrderedArr<T: Ord, const CAPACITY: usize> {
    pub arr: [Option<T>; CAPACITY],
    size: usize,
}

impl<T: Ord, const CAPACITY: usize> MinHeapOrderedArr<T, CAPACITY> {
    const INIT: Option<T> = None;

    pub fn new() -> Self {
        MinHeapOrderedArr {
            arr: [Self::INIT; CAPACITY],
            size: 0,
        }
    }
}

impl<T: Ord, const CAPACITY: usize> MinHeap<T> for MinHeapOrderedArr<T, CAPACITY> {
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("min heap full".to_string());
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
