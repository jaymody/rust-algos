use super::Stack;

pub struct StackFixedArray<T, const MAX_SIZE: usize> {
    arr: [Option<T>; MAX_SIZE],
    size: usize,
}

impl<T, const MAX_SIZE: usize> StackFixedArray<T, MAX_SIZE> {
    const INIT: Option<T> = None;

    pub fn new() -> Self {
        StackFixedArray {
            arr: [Self::INIT; MAX_SIZE],
            size: 0,
        }
    }
}

impl<T, const MAX_SIZE: usize> Stack<T> for StackFixedArray<T, MAX_SIZE> {
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= MAX_SIZE {
            return Err("capacity full".to_string());
        }
        self.arr[self.size] = Some(item);
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        self.arr[self.size].take()
    }

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
