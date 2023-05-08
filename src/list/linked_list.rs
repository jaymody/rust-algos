use std::{marker::PhantomData, ptr::null_mut};

use super::List;

/* types and structs */
type Link<T> = *mut Node<T>;

struct Node<T> {
    item: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new_link(item: T) -> Link<T> {
        Box::into_raw(Box::new(Node {
            item: item,
            prev: null_mut(),
            next: null_mut(),
        }))
    }
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: null_mut(),
            tail: null_mut(),
            size: 0,
        }
    }
}

/* impl List trait */
impl<T> List<T> for LinkedList<T> {
    fn push_front(&mut self, item: T) {
        unsafe {
            let mut new_head = Node::new_link(item);

            if self.is_empty() {
                self.tail = new_head;
            } else {
                (*new_head).next = self.head;
                (*self.head).prev = new_head;
            }

            self.head = new_head;
            self.size += 1;
        }
    }

    fn push_back(&mut self, item: T) {
        unsafe {
            let new_tail = Node::new_link(item);

            if self.is_empty() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
                (*new_tail).prev = self.tail;
            }

            self.tail = new_tail;
            self.size += 1;
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        unsafe {
            (!self.is_empty()).then(|| {
                let old_head = self.head;
                self.head = (*old_head).next;
                self.size -= 1;
                Box::from_raw(old_head).item
            })
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        unsafe {
            (!self.is_empty()).then(|| {
                let old_tail = self.tail;
                self.tail = (*old_tail).prev;
                self.size -= 1;
                Box::from_raw(old_tail).item
            })
        }
    }

    fn peek_front(&self) -> Option<&T> {
        unsafe { (!self.is_empty()).then(|| &(*self.head).item) }
    }

    fn peek_back(&self) -> Option<&T> {
        unsafe { (!self.is_empty()).then(|| &(*self.tail).item) }
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
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

/* impl IntoIterator<Item = &T> */
pub struct Iter<'a, T> {
    current: Link<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current.is_null() {
                None
            } else {
                let old = self.current;
                self.current = (*old).next;
                Some(&(*old).item)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }
}

/* impl IntoIterator<Item = &mut T> */
pub struct IterMut<'a, T> {
    current: Link<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current.is_null() {
                None
            } else {
                let old = self.current;
                self.current = (*old).next;
                Some(&mut (*old).item)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            current: self.head,
            _marker: PhantomData,
        }
    }
}
