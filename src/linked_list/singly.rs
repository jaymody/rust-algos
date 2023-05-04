use std::ops::{Deref, DerefMut};

use super::LinkedList;

/* node and link structs */
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(item: T, next: Link<T>) -> Self {
        Node {
            item: item,
            next: next,
        }
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

impl<T> LinkedList<T> for SinglyLinkedList<T> {
    fn push_front(&mut self, item: T) -> Result<(), String> {
        self.head = Some(Box::new(Node::new(item, self.head.take())));
        self.size += 1;
        Ok(())
    }

    fn push_back(&mut self, item: T) -> Result<(), String> {
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
        Ok(())
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
        Some(&self.head.as_ref()?.deref().item)
    }

    fn peek_back(&self) -> Option<&T> {
        let mut prev = self.head.as_ref()?.deref();
        while let Some(node) = prev.next.as_ref() {
            prev = node.as_ref();
        }
        Some(&prev.item)
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

/* tests */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut list = SinglyLinkedList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.size(), 0);

        // push front pop front
        list.push_front(1).unwrap();
        list.push_front(2).unwrap();
        assert_eq!(list.size(), 2);
        assert_eq!(list.is_empty(), false);
        list.push_front(3).unwrap();

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.size(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.size(), 0);
        assert_eq!(list.is_empty(), true);

        // push back pop front
        list.push_back(1).unwrap();
        list.push_back(2).unwrap();
        list.push_back(3).unwrap();

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);

        // push front pop back
        list.push_front(1).unwrap();
        list.push_front(2).unwrap();
        list.push_front(3).unwrap();

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);

        // push back pop back
        list.push_back(1).unwrap();
        list.push_back(2).unwrap();
        list.push_back(3).unwrap();

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);

        // peek front and back and then into iter
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_back(), None);

        list.push_back(1).unwrap();
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));

        list.push_back(2).unwrap();
        list.push_back(3).unwrap();

        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));

        for (i, x) in list.into_iter().enumerate() {
            assert_eq!(x, i + 1)
        }
    }
}
