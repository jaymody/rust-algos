use super::Queue;

/// Fixed length array implementation for a queue.
///
/// This implementation uses a fixed length array with a pointer for the start
/// of the queue, and a pointer to the end of the queue. The queue items are
/// kept contiguous in the array, if the array needs to grow past the end, we
/// simply wrap around back to the front (keeping it contiguous in terms of
/// wrapping around).
///
/// Push, pop, and peek are all O(1) operations.
pub struct QueueFixedArr<T, const CAPACITY: usize> {
    arr: [Option<T>; CAPACITY],
    size: usize,
    front_i: usize,
    back_i: usize,
}

impl<T, const CAPACITY: usize> QueueFixedArr<T, CAPACITY> {
    const INIT: Option<T> = None;

    pub fn new() -> Self {
        QueueFixedArr {
            arr: [Self::INIT; CAPACITY],
            size: 0,
            front_i: 0,
            back_i: 0,
        }
    }
}

impl<T, const CAPACITY: usize> Queue<T> for QueueFixedArr<T, CAPACITY> {
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("capacity full".to_string());
        }
        self.arr[self.back_i] = Some(item);
        self.size += 1;
        self.back_i = (self.back_i + 1) % CAPACITY;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let popped = self.arr[self.front_i].take();
        self.size -= 1;
        self.front_i = (self.front_i + 1) % CAPACITY;
        popped
    }

    fn peek(&self) -> Option<&T> {
        self.arr[self.front_i].as_ref()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
