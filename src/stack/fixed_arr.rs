use super::Stack;

/// Stack implementation using a fixed length array.
///
/// We push elements onto the end of the array, and pop elements from the end,
/// both in O(1) time.
pub struct StackFixedArray<T, const CAPACITY: usize> {
    arr: [Option<T>; CAPACITY],
    size: usize,
}

impl<T, const CAPACITY: usize> StackFixedArray<T, CAPACITY> {
    const INIT: Option<T> = None;

    pub fn new() -> Self {
        StackFixedArray {
            arr: [Self::INIT; CAPACITY],
            size: 0,
        }
    }
}

impl<T, const CAPACITY: usize> Stack<T> for StackFixedArray<T, CAPACITY> {
    /// Push an item onto the stack.
    ///
    /// ### Implementation
    /// Adds the item to the end of the array in O(1) time.
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("capacity full".to_string());
        }
        self.arr[self.size] = Some(item);
        self.size += 1;
        Ok(())
    }

    /// Pop an item from the stack (return None if the stack is empty).
    ///
    /// ### Implementation
    /// Removes the last item in the array in O(1) time.
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        self.arr[self.size].take()
    }

    /// Peek at the next item on the stack (return None if the stack is empty).
    ///
    /// ### Implementation
    /// Return a reference to the last item in the array in O(1) time.
    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
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
