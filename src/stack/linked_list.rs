use super::Stack;

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

/// Stack implementation using a singly linked list.
///
/// Pushing corresponds to inserting at the front of the list, and popping
/// corresponds to popping the front of the list. Both run in O(1) time.
pub struct StackLinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> StackLinkedList<T> {
    pub fn new() -> Self {
        StackLinkedList {
            head: None,
            size: 0,
        }
    }
}

impl<T> Stack<T> for StackLinkedList<T> {
    /// Push an item onto the stack.
    ///
    /// ### Implementation
    /// Push the item to the front of the linked list making it the new head.
    fn push(&mut self, item: T) -> Result<(), String> {
        self.head = Some(Box::new(Node::new(item, self.head.take())));
        self.size += 1;
        Ok(())
    }

    /// Pop an item from the stack (return None if the stack is empty).
    ///
    /// ### Implementation
    /// Pops the head of the list and reassigns head to head.next.
    fn pop(&mut self) -> Option<T> {
        let head = self.head.take()?;
        self.head = head.next;
        self.size -= 1;
        Some(head.item)
    }

    /// Peek at the next item on the stack (return None if the stack is empty).
    ///
    /// ### Implementation
    /// Return a reference to the head item.
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
