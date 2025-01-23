use std::{marker::PhantomData, ptr::null_mut};

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

/// A doubly linked list.
///
/// Since rust is safe by default, and only one mutable reference can exist
/// at any given point in time, this implementation requires unsafe rust via
/// raw pointers *mut Node<T> for prev, next, head, and tail.
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: null_mut(),
            tail: null_mut(),
            len: 0,
        }
    }

    pub fn push_front(&mut self, item: T) {
        unsafe {
            let new_head = Node::new_link(item);

            if self.is_empty() {
                self.tail = new_head;
            } else {
                (*new_head).next = self.head;
                (*self.head).prev = new_head;
            }

            self.head = new_head;
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, item: T) {
        unsafe {
            let new_tail = Node::new_link(item);

            if self.is_empty() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
                (*new_tail).prev = self.tail;
            }

            self.tail = new_tail;
            self.len += 1;
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        if index > self.len {
            panic!("index out of bounds (idx = {}, len = {})", index, self.len);
        }

        if index == 0 {
            self.push_front(item)
        } else if index == self.len {
            self.push_back(item)
        } else {
            self.len += 1;
            let new_node = Node::new_link(item);
            unsafe {
                let mut node = self.head;
                for _ in 0..index - 1 {
                    node = (*node).next;
                }

                (*new_node).next = (*node).next;
                (*new_node).prev = node;

                (*(*node).next).prev = new_node;
                (*node).next = new_node;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            (!self.is_empty()).then(|| {
                let old_head = self.head;
                self.head = (*old_head).next;
                if !self.head.is_null() {
                    (*self.head).prev = null_mut();
                }
                self.len -= 1;
                Box::from_raw(old_head).item
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            (!self.is_empty()).then(|| {
                let old_tail = self.tail;
                self.tail = (*old_tail).prev;
                if !self.tail.is_null() {
                    (*self.tail).next = null_mut();
                }
                self.len -= 1;
                Box::from_raw(old_tail).item
            })
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("index out of bounds (idx = {}, len = {})", index, self.len);
        }

        if index == 0 {
            self.pop_front().unwrap()
        } else if index == self.len - 1 {
            self.pop_back().unwrap()
        } else {
            self.len -= 1;
            let mut node = self.head;
            unsafe {
                for _ in 0..index {
                    node = (*node).next;
                }

                let next = (*node).next;
                let prev = (*node).prev;

                (*prev).next = next;
                (*next).prev = prev;

                Box::from_raw(node).item
            }
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        unsafe { (!self.is_empty()).then(|| &(*self.head).item) }
    }

    pub fn peek_back(&self) -> Option<&T> {
        unsafe { (!self.is_empty()).then(|| &(*self.tail).item) }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.len(), 0);

        // push front pop front
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.is_empty(), false);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.is_empty(), true);

        // push back pop front
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);

        // push front pop back
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);

        // push back pop back
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);

        // peek front and back
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);

        list.push_back(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));

        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));

        // test iters
        for x in &mut list {
            *x += 10;
        }

        for (i, x) in (&list).into_iter().enumerate() {
            assert_eq!(x, &(i + 11))
        }

        for (i, x) in list.into_iter().enumerate() {
            assert_eq!(x, i + 11)
        }
    }

    #[test]
    fn test_insert_remove() {
        let mut list = LinkedList::new();
        list.insert(0, -4);
        list.insert(1, -5);
        list.insert(0, -1);
        list.insert(1, -2);
        list.insert(2, -3);

        let mut iter = (&list).into_iter();
        assert_eq!(iter.next(), Some(&-1));
        assert_eq!(iter.next(), Some(&-2));
        assert_eq!(iter.next(), Some(&-3));
        assert_eq!(iter.next(), Some(&-4));
        assert_eq!(iter.next(), Some(&-5));
        assert_eq!(iter.next(), None);

        assert_eq!(list.remove(2), -3);
        assert_eq!(list.remove(3), -5);
        assert_eq!(list.remove(0), -1);
        assert_eq!(list.remove(1), -4);

        assert_eq!(list.peek_front(), Some(&-2));
        assert_eq!(list.peek_back(), Some(&-2));
    }

    #[test]
    #[should_panic(expected = "index out of bounds (idx = 1, len = 0)")]
    fn test_insert_out_of_bounds_1() {
        let mut list = LinkedList::new();
        list.insert(1, -10);
    }

    #[test]
    #[should_panic(expected = "index out of bounds (idx = 4, len = 3)")]
    fn test_insert_out_of_bounds_2() {
        let mut list = LinkedList::new();
        list.insert(0, 1);
        list.insert(0, 2);
        list.insert(2, 3);

        let mut iter = (&list).into_iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        list.insert(4, 10);
    }

    #[test]
    #[should_panic(expected = "index out of bounds (idx = 0, len = 0)")]
    fn test_remove_out_of_bounds_1() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.remove(0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds (idx = 2, len = 2)")]
    fn test_remove_out_of_bounds_2() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.remove(2), 3);
        assert_eq!(list.remove(0), 1);
        assert_eq!(list.remove(2), 5);

        let mut iter = (&list).into_iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);

        list.remove(2);
    }
}
