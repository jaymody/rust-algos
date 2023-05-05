use std::ptr::null_mut;

use super::LinkedList;

/* types and structs */
type Link<T> = *mut Node<T>;

struct Node<T> {
    item: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(item: T, prev: Link<T>, next: Link<T>) -> Self {
        Node { item, prev, next }
    }
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: null_mut(),
            tail: null_mut(),
            size: 0,
        }
    }
}

/* impl LinkedList trait */
impl<T> LinkedList<T> for DoublyLinkedList<T> {
    fn push_front(&mut self, item: T) {
        let mut new_head = Node::new(item, null_mut(), null_mut());

        if self.head.is_null() {
            self.tail = &mut new_head;
        } else {
            new_head.next = self.head;
        }

        self.head = &mut new_head;
        self.size += 1;
    }

    fn push_back(&mut self, item: T) {
        todo!()
    }

    fn pop_front(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let old_head = self.head;
                self.head = (*old_head).next;
                self.size -= 1;
                Some(Box::from_raw(old_head).item)
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    fn peek_front(&self) -> Option<&T> {
        todo!()
    }

    fn peek_back(&self) -> Option<&T> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

/* impl IntoIterator<Item = T> */
pub struct IntoIter<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

/* impl IntoIterator<Item = &T> */
pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

/* impl IntoIterator<Item = &mut T> */
pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a mut DoublyLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
