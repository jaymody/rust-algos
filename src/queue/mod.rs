mod linked_list;

pub use linked_list::QueueLinkedList;

/// A first-in-first-out (FIFO) queue.
pub trait Queue<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut list = QueueLinkedList::new();
        assert_eq!(list.pop(), None);
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.size(), 0);
        assert_eq!(list.peek(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));

        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&1));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }
}
