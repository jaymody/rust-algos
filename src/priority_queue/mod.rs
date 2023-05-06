pub mod heap;
pub mod ordered_arr;
pub mod unordered_arr;

pub use heap::PriorityQueueBinaryHeap;
pub use ordered_arr::PriorityQueueOrderedArr;
pub use unordered_arr::PriorityQueueUnorderedArr;

pub trait PriorityQueue<T: Ord> {
    fn push(&mut self, item: T) -> Result<(), String>;
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(pq: &mut dyn PriorityQueue<i32>) {
        assert_eq!(pq.is_empty(), true);
        assert_eq!(pq.size(), 0);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.peek(), None);

        pq.push(4).unwrap();
        pq.push(2).unwrap();
        pq.push(6).unwrap();
        pq.push(3).unwrap();

        assert_eq!(pq.peek(), Some(&2));
        assert_eq!(pq.pop(), Some(2));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.size(), 2);
        assert_eq!(pq.pop(), Some(4));
        assert_eq!(pq.size(), 1);
        assert_eq!(pq.peek(), Some(&6));
        assert_eq!(pq.pop(), Some(6));
        assert_eq!(pq.size(), 0);
        assert_eq!(pq.pop(), None);
        assert_eq!(pq.size(), 0);
        assert_eq!(pq.peek(), None);

        pq.push(1).unwrap();
        pq.push(2).unwrap();
        pq.push(2).unwrap();
        pq.push(-1).unwrap();

        assert_eq!(pq.size(), 4);
        assert_eq!(pq.peek(), Some(&-1));
        assert_eq!(pq.pop(), Some(-1));
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(2));
        pq.push(0).unwrap();
        pq.push(3).unwrap();
        assert_eq!(pq.pop(), Some(0));
        assert_eq!(pq.pop(), Some(2));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_unordered_arr() {
        let mut pq: PriorityQueueUnorderedArr<i32, 99> = PriorityQueueUnorderedArr::new();
        test(&mut pq);

        let mut pq: PriorityQueueUnorderedArr<i32, 5> = PriorityQueueUnorderedArr::new();
        assert!(pq.push(1).is_ok());
        assert!(pq.push(2).is_ok());
        assert!(pq.push(3).is_ok());
        assert!(pq.push(4).is_ok());
        assert!(pq.push(5).is_ok());
        assert!(pq.push(6).is_err());
    }

    #[test]
    fn test_ordered_arr() {
        let mut pq: PriorityQueueOrderedArr<i32, 99> = PriorityQueueOrderedArr::new();
        test(&mut pq);

        let mut pq: PriorityQueueOrderedArr<i32, 5> = PriorityQueueOrderedArr::new();
        assert!(pq.push(1).is_ok());
        assert!(pq.push(2).is_ok());
        assert!(pq.push(3).is_ok());
        assert!(pq.push(4).is_ok());
        assert!(pq.push(5).is_ok());
        assert!(pq.push(6).is_err());
    }

    #[test]
    fn test_binary_heap() {
        let mut pq: PriorityQueueBinaryHeap<i32, 99> = PriorityQueueBinaryHeap::new();
        test(&mut pq);

        let mut pq: PriorityQueueBinaryHeap<i32, 5> = PriorityQueueBinaryHeap::new();
        assert!(pq.push(1).is_ok());
        assert!(pq.push(2).is_ok());
        assert!(pq.push(3).is_ok());
        assert!(pq.push(4).is_ok());
        assert!(pq.push(5).is_ok());
        assert!(pq.push(6).is_err());
    }
}
