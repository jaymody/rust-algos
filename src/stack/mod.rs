mod fixed_arr;
mod linked_list;

pub use fixed_arr::StackFixedArray;
pub use linked_list::StackLinkedList;

/// A last-in-first-out (LIFO) stack.
pub trait Stack<T> {
    fn push(&mut self, item: T) -> Result<(), String>;
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_stack(stack: &mut dyn Stack<i32>) {
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);

        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        stack.push(4).unwrap();

        assert_eq!(stack.size(), 4);
        assert_eq!(stack.is_empty(), false);

        assert_eq!(stack.peek(), Some(&4));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        assert_eq!(stack.size(), 1);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.peek(), Some(&1));

        stack.push(5).unwrap();
        stack.push(2).unwrap();

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(1));

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn test_stack_linked_list() {
        test_stack(&mut StackLinkedList::new());
    }

    #[test]
    fn test_stack_fixed_arr() {
        test_stack(&mut StackFixedArray::<i32, 50>::new());

        // test size limit
        let mut stack = StackFixedArray::<i32, 5>::new();
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert!(stack.push(3).is_ok());
        assert!(stack.push(4).is_ok());
        assert!(stack.push(5).is_ok());
        assert!(stack.push(6).is_err());
    }
}
