pub mod ordered_arr;
pub mod unordered_arr;

pub use ordered_arr::MinHeapOrderedArr;
pub use unordered_arr::MinHeapUnorderedArr;

pub trait MinHeap<T: Ord> {
    fn push(&mut self, item: T) -> Result<(), String>;
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(heap: &mut dyn MinHeap<i32>) {
        assert_eq!(heap.is_empty(), true);
        assert_eq!(heap.size(), 0);
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.peek(), None);

        heap.push(4).unwrap();
        heap.push(2).unwrap();
        heap.push(6).unwrap();
        heap.push(3).unwrap();

        assert_eq!(heap.peek(), Some(&2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.size(), 2);
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.size(), 1);
        assert_eq!(heap.peek(), Some(&6));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.size(), 0);
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.size(), 0);
        assert_eq!(heap.peek(), None);

        heap.push(1).unwrap();
        heap.push(2).unwrap();
        heap.push(2).unwrap();
        heap.push(-1).unwrap();

        assert_eq!(heap.size(), 4);
        assert_eq!(heap.peek(), Some(&-1));
        assert_eq!(heap.pop(), Some(-1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        heap.push(0).unwrap();
        heap.push(3).unwrap();
        assert_eq!(heap.pop(), Some(0));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_unordered_arr() {
        let mut heap: MinHeapUnorderedArr<i32, 99> = MinHeapUnorderedArr::new();
        test(&mut heap);

        let mut heap: MinHeapUnorderedArr<i32, 5> = MinHeapUnorderedArr::new();
        assert!(heap.push(1).is_ok());
        assert!(heap.push(2).is_ok());
        assert!(heap.push(3).is_ok());
        assert!(heap.push(4).is_ok());
        assert!(heap.push(5).is_ok());
        assert!(heap.push(6).is_err());
    }

    #[test]
    fn test_ordered_arr() {
        let mut heap: MinHeapOrderedArr<i32, 99> = MinHeapOrderedArr::new();
        test(&mut heap);

        let mut heap: MinHeapOrderedArr<i32, 5> = MinHeapOrderedArr::new();
        assert!(heap.push(1).is_ok());
        assert!(heap.push(2).is_ok());
        assert!(heap.push(3).is_ok());
        assert!(heap.push(4).is_ok());
        assert!(heap.push(5).is_ok());
        assert!(heap.push(6).is_err());
    }
}
