use std::{fmt::Debug, ops::DerefMut, ptr::null_mut};

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
impl<T> Queue<T> for QueueLinkedList<T>
where
    T: Debug,
{
    fn enqueue(&mut self, item: T) {
        unsafe {
            let mut new_node = Box::new(Node::new(item, None));

            match self.head.take() {
                None => {
                    self.tail = &mut *new_node;
                    self.head = Some(new_node);
                    println!("hi: {:?}", (*self.tail).item);
                }
                head => {
                    self.head = head;
                    (*self.tail).next = Some(new_node);
                    self.tail = (*self.tail).next.as_mut().unwrap().deref_mut();
                }
            };
        }

        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        let old_head = self.head.take()?;
        self.size -= 1;
        match old_head.next {
            None => {
                self.tail = null_mut();
                Some(old_head.item)
            }
            node => {
                self.head = node;
                Some(old_head.item)
            }
        }
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
