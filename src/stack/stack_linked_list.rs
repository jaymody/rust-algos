use super::Stack;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Link<T>) -> Self {
        Node {
            data: data,
            next: next,
        }
    }
}

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
    fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node::new(data, self.head.take())));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.size -= !self.head.is_none() as usize;
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
