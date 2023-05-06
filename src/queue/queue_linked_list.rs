use std::{ops::DerefMut, ptr::null_mut};

use super::Queue;

/* node and link structs */
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(item: T, next: Link<T>) -> Self {
        Node { item, next }
    }
}

pub struct QueueLinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    size: usize,
}

impl<T> QueueLinkedList<T> {
    pub fn new() -> Self {
        QueueLinkedList {
            head: None,
            tail: null_mut(),
            size: 0,
        }
    }
}

/* impl Queue trait */
impl<T> Queue<T> for QueueLinkedList<T> {
    fn push(&mut self, item: T) {
        unsafe {
            let mut new_tail = Box::new(Node::new(item, None));

            if self.tail.is_null() {
                self.tail = &mut *new_tail;
                self.head = Some(new_tail);
            } else {
                (*self.tail).next = Some(new_tail);
                self.tail = (*self.tail).next.as_mut().unwrap().deref_mut();
            }

            self.size += 1;
        }
    }

    fn pop(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.size -= 1;

        if old_head.next.is_none() {
            self.tail = null_mut()
        } else {
            self.head = old_head.next
        }

        Some(old_head.item)
    }

    fn peek_front(&self) -> Option<&T> {
        Some(&self.head.as_ref()?.item)
    }

    fn peek_back(&self) -> Option<&T> {
        unsafe {
            if !self.tail.is_null() {
                Some(&(*self.tail).item)
            } else {
                None
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
