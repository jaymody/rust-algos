mod doubly;

pub use doubly::DoublyLinkedList;

/// List data structure that can hold a variable number of items.
///
/// TODO: add various functions like appending one list to another, peek_at,
/// pop_at, push_at, double ended iterators, reverse, truncate, get slice, etc
/// ...
pub trait List<T>
where
    Self: IntoIterator<Item = T>,
    for<'a> &'a Self: IntoIterator<Item = &'a T>,
    for<'a> &'a mut Self: IntoIterator<Item = &'a mut T>,
{
    fn push_front(&mut self, item: T);
    fn push_back(&mut self, item: T);
    // fn push_at(&mut self, index: usize, item: T);

    fn pop_front(&mut self) -> Option<T>;
    fn pop_back(&mut self) -> Option<T>;
    // fn pop_at();

    fn peek_front(&self) -> Option<&T>;
    fn peek_back(&self) -> Option<&T>;
    // fn peek_at();

    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;

    // fn reverse(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubly() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.is_empty(), true);
        assert_eq!(list.size(), 0);

        // push front pop front
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.size(), 2);
        assert_eq!(list.is_empty(), false);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.size(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.size(), 0);
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
}
