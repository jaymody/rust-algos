pub mod queue_linked_list;
mod test;

pub use queue_linked_list::QueueLinkedList;

pub trait Queue<T> {
    fn enqueue(&mut self, item: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek_front(&self) -> Option<&T>;
    fn peek_back(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}
