mod binary_search_tree;
mod ordered_arr;
mod red_black_bst;

pub use binary_search_tree::BinarySearchTree;
pub use ordered_arr::OrderedArrST;
pub use red_black_bst::RedBlackBST;

/// Keys must be Clone, Copy, and Ord (i.e. defines ==, =>, <=, >, <).
/// Technically only == is needed for a regular non-ordered Symbol Table,
/// however for simplicity/consistency with OrderedSymbolTable, we'll leave this
/// as is.
///
/// The Clone + Copy are so we can pass around Key by reference, which is a lot
/// more convenient than passing around by reference. This is probably fine
/// since the key types that are likely to be used will be integers or strings,
/// which implement Clone + Copy.

pub trait KeyT: Ord + Clone + Copy {}
impl<T: Ord + Clone + Copy> KeyT for T {}

/// Symbol tables stores key-value pairs.
///
/// See: https://algs4.cs.princeton.edu/31elementary/
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

/// Ordered symbol tables store key-value pairs while also supporting
/// ordered operations (like finding the min key, max key, etc ...).
///
/// See: https://algs4.cs.princeton.edu/31elementary/
pub trait OrderedSymbolTable<K: KeyT, V>: SymbolTable<K, V>
where
    for<'a> &'a Self: IntoIterator<Item = &'a K>,
{
    fn min(&self) -> Option<K>;
    fn max(&self) -> Option<K>;
    fn floor(&self, key: K) -> Option<K>;
    fn ceil(&self, key: K) -> Option<K>;
    fn rank(&self, key: K) -> usize;
    fn select(&self, rank: usize) -> Option<K>;
    fn pop_min(&self) -> Option<(K, V)>;
    fn pop_max(&self) -> Option<(K, V)>;
}
