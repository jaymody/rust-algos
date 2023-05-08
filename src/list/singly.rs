use std::ops::DerefMut;

use super::List;

/* node and link structs */
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(item: T, next: Link<T>) -> Self {
        Node { item, next }
    }
}

/* singly linked list struct */
pub struct SinglyLinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList {
            head: None,
            size: 0,
        }
    }
}

/* impl List trait */
impl<T> List<T> for SinglyLinkedList<T> {
    fn push_front(&mut self, item: T) {
        self.head = Some(Box::new(Node::new(item, self.head.take())));
        self.size += 1;
    }

    fn push_back(&mut self, item: T) {
        let new_node = Some(Box::new(Node::new(item, None)));

        match self.head.as_mut() {
            None => self.head = new_node,
            Some(node) => {
                let mut prev = node.deref_mut();
                loop {
                    if prev.next.is_none() {
                        break;
                    }
                    prev = prev.next.as_mut().unwrap();
                }
                prev.next = new_node;
            }
        }

        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take()?;
        self.head = head.next;
        self.size -= 1;
        Some(head.item)
    }

    fn pop_back(&mut self) -> Option<T> {
        let head = self.head.take()?;

        self.size -= 1;
        if head.next.is_none() {
            self.head = None;
            Some(head.item)
        } else {
            self.head = Some(head);

            let mut prev = self.head.as_mut().unwrap();
            while prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
                prev = prev.next.as_mut().unwrap();
            }
            let tail = prev.next.take().unwrap();
            prev.next = None;
            Some(tail.item)
        }
    }

    fn peek_front(&self) -> Option<&T> {
        self.into_iter().next()
    }

    fn peek_back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.into_iter().nth(self.size - 1)
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
    list: SinglyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for SinglyLinkedList<T> {
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
        let current = self.current?;
        self.current = current.next.as_deref();
        Some(&current.item)
    }
}

impl<'a, T> IntoIterator for &'a SinglyLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

/* impl IntoIterator<Item = &mut T> */
pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            None => None,
            Some(node) => {
                self.current = node.next.as_deref_mut();
                Some(&mut node.item)
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a mut SinglyLinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            current: self.head.as_deref_mut(),
        }
    }
}
