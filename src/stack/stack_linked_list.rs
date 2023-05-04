use super::Stack;
use crate::linked_list::{LinkedList, SinglyLinkedList};

pub struct StackLinkedList<T> {
    list: SinglyLinkedList<T>,
}

impl<T> StackLinkedList<T> {
    pub fn new() -> Self {
        StackLinkedList {
            list: SinglyLinkedList::new(),
        }
    }
}

impl<T> Stack<T> for StackLinkedList<T> {
    fn push(&mut self, item: T) -> Result<(), String> {
        self.list.push_front(item);
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.list.peek_front()
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn size(&self) -> usize {
        self.list.size()
    }
}
