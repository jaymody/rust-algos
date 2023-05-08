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

/// A singly linked list queue implementation.
///
/// In addition to the head, a tail reference is also kept so that pushing to
/// the queue remains an O(1) operation, in addition to popping and peeking.
///
/// Because only one variable can own a given object at a time in rust, the tail
/// is implemented with unsafe rust using a raw pointer.
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
    /// Push an item to the queue.
    ///
    /// ### Implementation
    /// We add new items to the end of the linked list.  If the linked list is
    /// empty, we simply set `head = new_node` and then have tail point to head.
    /// Otherwise, set `tail.next = new_node` and then update tail to point to
    /// the new node. This can all be done in O(1) time.
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

    /// Pop the next item from the queue (None if queue is empty).
    ///
    /// ### Implementation
    /// The next item from the queue is always the head of the queue. So we
    /// set `head = head.next` and then return the old head.
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

    /// Peek at the next item in the queue (None if queue is empty).
    ///
    /// ### Implementation
    /// The next item from the queue is always the head of the queue, so we
    /// just return a reference to the head item.
    fn peek(&self) -> Option<&T> {
        Some(&self.head.as_ref()?.item)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
