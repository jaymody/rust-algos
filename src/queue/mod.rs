mod fixed_arr;
mod linked_list;

pub use fixed_arr::QueueFixedArr;
pub use linked_list::QueueLinkedList;

/// A first-in-first-out (FIFO) queue.
pub trait Queue<T> {
    fn push(&mut self, item: T) -> Result<(), String>;
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(queue: &mut dyn Queue<i32>) {
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.is_empty(), true);
        assert_eq!(queue.size(), 0);
        assert_eq!(queue.peek(), None);

        queue.push(1).unwrap();
        assert_eq!(queue.peek(), Some(&1));

        queue.push(2).unwrap();
        queue.push(3).unwrap();
        assert_eq!(queue.peek(), Some(&1));

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);

        assert_eq!(queue.peek(), None);
        queue.push(10).unwrap();
        assert_eq!(queue.peek(), Some(&10));
        queue.push(5).unwrap();
        assert_eq!(queue.peek(), Some(&10));
        assert_eq!(queue.pop(), Some(10));
        assert_eq!(queue.peek(), Some(&5));
        queue.push(4).unwrap();
        queue.push(3).unwrap();
        queue.push(2).unwrap();
        assert_eq!(queue.peek(), Some(&5));
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.peek(), Some(&2));
        queue.push(1).unwrap();
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_queue_linked_list() {
        test(&mut QueueLinkedList::new());
    }

    #[test]
    fn test_queue_fixed_arr() {
        test(&mut QueueFixedArr::<i32, 4>::new());

        let mut queue = QueueFixedArr::<i32, 5>::new();
        assert!(queue.push(1).is_ok());
        assert!(queue.push(2).is_ok());
        assert!(queue.push(3).is_ok());
        assert!(queue.push(4).is_ok());
        assert!(queue.push(5).is_ok());
        assert!(queue.push(6).is_err());
    }
}
