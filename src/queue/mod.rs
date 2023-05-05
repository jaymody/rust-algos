pub mod queue_linked_list;

pub use queue_linked_list::QueueLinkedList;

pub trait Queue<T> {
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek_front(&self) -> Option<&T>;
    fn peek_back(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut list = QueueLinkedList::new();
        assert_eq!(list.dequeue(), None);
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.size(), 0);
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);

        list.enqueue(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));

        list.enqueue(2);
        list.enqueue(3);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));

        assert_eq!(list.dequeue(), Some(1));
        assert_eq!(list.dequeue(), Some(2));
        assert_eq!(list.dequeue(), Some(3));
        assert_eq!(list.dequeue(), None);
    }
}
