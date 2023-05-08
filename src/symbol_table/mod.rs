mod binary_search_tree;
mod ordered_arr;

use std::fmt::Debug;

pub use binary_search_tree::BinarySearchTree;
pub use ordered_arr::OrderedArrST;

pub trait KeyT: Ord + Clone + Copy + Debug {}
impl<T: Ord + Clone + Copy + Debug> KeyT for T {}

pub trait SymbolTable<K: KeyT, V>
where
    for<'a> &'a Self: IntoIterator<Item = &'a K>,
{
    fn put(&mut self, key: K, val: V) -> Result<(), String>;
    fn get(&self, key: K) -> Option<&V>;
    fn pop(&mut self, key: K) -> Option<V>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_arr_st() {
        let mut st: OrderedArrST<&str, i32, 5> = OrderedArrST::new();
        assert_eq!(st.is_empty(), true);
        assert_eq!(st.size(), 0);

        st.put("apple", 10).unwrap();
        st.put("banana", 5).unwrap();
        st.put("cat", -3).unwrap();
        st.put("dog", 0).unwrap();
        assert_eq!(st.size(), 4);
        assert_eq!(st.is_empty(), false);

        assert_eq!(st.get("apple"), Some(&10));
        assert_eq!(st.get("banana"), Some(&5));
        assert_eq!(st.get("cat"), Some(&-3));
        assert_eq!(st.get("dog"), Some(&0));
        assert_eq!(st.get("elephant"), None);

        assert_eq!(st.pop("cat"), Some(-3));
        assert_eq!(st.get("cat"), None);
        assert_eq!(st.pop("cat"), None);
        assert_eq!(st.get("dog"), Some(&0));

        st.put("apple", 20).unwrap();
        assert_eq!(st.get("apple"), Some(&20));
        assert_eq!(st.size(), 3);

        assert!(st.put("a", 1).is_ok());
        assert!(st.put("b", 1).is_ok());
        assert!(st.put("a", 1).is_ok());
        assert!(st.put("c", 1).is_err());

        let mut iter = (&st).into_iter();
        assert_eq!(iter.next(), Some(&"a"));
        assert_eq!(iter.next(), Some(&"apple"));
        assert_eq!(iter.next(), Some(&"b"));
        assert_eq!(iter.next(), Some(&"banana"));
        assert_eq!(iter.next(), Some(&"dog"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_binary_search_tree() {
        // test 1
        let mut st = BinarySearchTree::new();
        assert_eq!(st.is_empty(), true);
        assert_eq!(st.size(), 0);

        st.put("apple", 10).unwrap();
        st.put("banana", 5).unwrap();
        st.put("cat", -3).unwrap();
        st.put("dog", 0).unwrap();
        assert_eq!(st.size(), 4);
        assert_eq!(st.is_empty(), false);

        assert_eq!(st.get("apple"), Some(&10));
        assert_eq!(st.get("banana"), Some(&5));
        assert_eq!(st.get("cat"), Some(&-3));
        assert_eq!(st.get("dog"), Some(&0));
        assert_eq!(st.get("elephant"), None);

        assert_eq!(st.pop("cat"), Some(-3));
        assert_eq!(st.get("cat"), None);
        assert_eq!(st.pop("cat"), None);
        assert_eq!(st.get("dog"), Some(&0));

        st.put("apple", 20).unwrap();
        assert_eq!(st.get("apple"), Some(&20));
        assert_eq!(st.size(), 3);

        let mut iter = (&st).into_iter();
        assert_eq!(iter.next(), Some(&"apple"));
        assert_eq!(iter.next(), Some(&"banana"));
        assert_eq!(iter.next(), Some(&"dog"));
        assert_eq!(iter.next(), None);

        // test 2
        let mut st = BinarySearchTree::new();
        st.put(5, -5).unwrap();
        st.put(2, -2).unwrap();
        st.put(3, 100000).unwrap();
        st.put(1, -1).unwrap();
        st.put(0, 0).unwrap();
        st.put(10, -10).unwrap();
        st.put(11, -11).unwrap();
        st.put(12, -12).unwrap();
        st.put(8, -8).unwrap();
        st.put(7, -7).unwrap();
        st.put(3, -3).unwrap();
        st.put(4, -4).unwrap();
        st.put(6, -6).unwrap();
        st.put(9, -9).unwrap();

        for i in 0..=12 {
            assert_eq!(st.get(i), Some(&-i));
        }

        assert!(st.pop(12).is_some());
        assert!(st.pop(5).is_some());
        assert!(st.pop(8).is_some());
        assert!(st.pop(0).is_some());
        assert!(st.pop(3).is_some());

        let mut iter = (&st).into_iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);
    }
}
