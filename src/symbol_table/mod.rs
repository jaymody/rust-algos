mod binary_search_tree;
mod ordered_arr;

pub use binary_search_tree::BinarySearchTree;
pub use ordered_arr::OrderedArrST;

pub trait KeyT: Ord + Clone + Copy {}
impl<T: Ord + Clone + Copy> KeyT for T {}

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

        st.put("apple", 20).unwrap();
        assert_eq!(st.get("apple"), Some(&20));
        assert_eq!(st.size(), 3);

        assert!(st.put("a", 1).is_ok());
        assert!(st.put("b", 1).is_ok());
        assert!(st.put("a", 1).is_ok());
        assert!(st.put("c", 1).is_err());
    }

    #[test]
    fn test_binary_search_tree() {
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

        st.put("apple", 20).unwrap();
        assert_eq!(st.get("apple"), Some(&20));
        assert_eq!(st.size(), 3);
    }
}
