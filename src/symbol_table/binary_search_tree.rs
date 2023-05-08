use std::ops::Deref;

use crate::stack::{Stack, StackLinkedList};

use super::{KeyT, SymbolTable};

type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

/// A binary search tree that stores key-value pairs.
///
/// A binary search tree is a binary tree with the constraint that for any
/// given node in the tree, all nodes in the right subtree are >= to it, and all
/// nodes in the left subtree are <= to it. If the tree is balanced, searching
/// and inserting into the tree basically corresponds to binary search, and is
/// an O(log n) operation. If the tree is not balanced, these operations become
/// O(n).
///
/// See: https://algs4.cs.princeton.edu/32bst/
pub struct BinarySearchTree<K: KeyT, V> {
    root: Link<K, V>,
    size: usize,
}

impl<K: KeyT, V> BinarySearchTree<K, V> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    fn insert(&mut self, node_to_insert: Node<K, V>) {
        fn visit<K: KeyT, V>(link: &mut Link<K, V>, node_to_insert: Node<K, V>) -> Link<K, V> {
            match link.take() {
                None => Some(Box::new(node_to_insert)),
                Some(mut node) => {
                    if node_to_insert.key == node.key {
                        node.val = node_to_insert.val;
                    } else if node_to_insert.key > node.key {
                        node.right = visit(&mut node.right, node_to_insert);
                    } else {
                        node.left = visit(&mut node.left, node_to_insert);
                    }
                    Some(node)
                }
            }
        }

        let mut root = self.root.take();
        self.root = visit(&mut root, node_to_insert);
        self.size += 1;
    }

    fn search(&self, key: K) -> Option<&Node<K, V>> {
        fn visit<'a, K: KeyT, V>(link: &'a Link<K, V>, key: K) -> Option<&'a Node<K, V>> {
            match link {
                None => None,
                Some(node) => {
                    if key == node.key {
                        Some(node.deref())
                    } else if key > node.key {
                        visit(&node.right, key)
                    } else {
                        visit(&node.left, key)
                    }
                }
            }
        }

        visit(&self.root, key)
    }

    fn delete(&mut self, key: K) -> Option<Node<K, V>> {
        fn visit<K: KeyT, V>(mut node: Node<K, V>, key: K) -> (Link<K, V>, Option<Node<K, V>>) {
            if key == node.key {
                match node.right.take() {
                    None => (node.left.take(), Some(node)),
                    Some(right) => {
                        let (right, mut successor) = find_successor(*right);
                        successor.left = node.left.take();
                        successor.right = right;
                        (Some(Box::new(successor)), Some(node))
                    }
                }
            } else if key > node.key {
                let mut deleted_node = None;
                if let Some(right) = node.right {
                    (node.right, deleted_node) = visit(*right, key);
                };
                (Some(Box::new(node)), deleted_node)
            } else {
                let mut deleted_node = None;
                if let Some(left) = node.left {
                    (node.left, deleted_node) = visit(*left, key);
                };
                (Some(Box::new(node)), deleted_node)
            }
        }

        fn find_successor<K: Ord + Copy + Clone, V>(
            mut node: Node<K, V>,
        ) -> (Link<K, V>, Node<K, V>) {
            match node.left {
                None => (None, node),
                Some(left) => {
                    let successor;
                    (node.left, successor) = find_successor(*left);
                    (Some(Box::new(node)), successor)
                }
            }
        }

        match self.root.take() {
            None => None,
            Some(root) => {
                let deleted_node;
                (self.root, deleted_node) = visit(*root, key);
                self.size -= 1;
                deleted_node
            }
        }
    }
}

impl<K: KeyT, V> SymbolTable<K, V> for BinarySearchTree<K, V> {
    /// Add (or update) the key-value pair.
    ///
    /// ### Implementation
    /// Traverse the tree until we find a match (update the value) or we reach
    /// a null node (replace the null node with a new entry). Put is O(log n)
    /// on average (if the tree is roughly balanced) and O(n) in the worst case
    /// (if the tree is very imbalanced).
    fn put(&mut self, key: K, val: V) -> Result<(), String> {
        self.insert(Node {
            key: key,
            val: val,
            left: None,
            right: None,
        });
        Ok(())
    }

    /// Get a reference to the value for the associated key (None if the key
    /// does not exist).
    ///
    /// ### Implementation
    /// Traverse the tree until we find a match (in which case, return a
    /// reference to the value) or we hit null (in which case, the key does
    /// not exist so we return None). Get is O(log n) on average (if the tree is
    /// roughly balanced) and O(n) in the worst case (if the tree is very
    /// imbalanced).
    fn get(&self, key: K) -> Option<&V> {
        Some(&self.search(key)?.val)
    }

    /// Remove the entry that matches the key, and return it's value (return
    /// None if the key does not exist).
    ///
    /// ### Implementation
    /// Traverse the tree until either we find a match, or we hit a null node.
    /// If we hit a null node, we simply return None since the key does not
    /// exist in our tree. If we find a match, we either:
    ///
    ///   1) replace it with the left node, if a right node does not exist
    ///   2) if a right node exists, we replace it with it's "successor"
    ///
    /// The successor is the next node in the tree (by value), as if you were
    /// traversing the tree in sorted order. This corresponds to the leftmost
    /// node in the right subtree. See the "delete" section in:
    /// https://algs4.cs.princeton.edu/32bst/
    ///
    /// Delete is O(log n) on average (if the tree is roughly balanced) and O(n)
    /// in the worst case (if the tree is very imbalanced).
    fn pop(&mut self, key: K) -> Option<V> {
        Some(self.delete(key)?.val)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

pub struct IntoIter<'a, K: KeyT, V> {
    pub stack: StackLinkedList<&'a Node<K, V>>,
}

impl<'a, K: KeyT, V> IntoIter<'a, K, V> {
    pub fn new(tree: &'a BinarySearchTree<K, V>) -> Self {
        let mut iter = IntoIter {
            stack: StackLinkedList::new(),
        };
        iter.push_left_nodes(&tree.root);
        iter
    }

    pub fn push_left_nodes(&mut self, mut link: &'a Link<K, V>) {
        while let Some(node) = link {
            self.stack.push(node.deref()).unwrap();
            link = &node.left;
        }
    }
}

impl<'a, K: KeyT, V> Iterator for IntoIter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(node) => {
                self.push_left_nodes(&node.right);
                Some(&node.key)
            }
        }
    }
}

impl<'a, K: KeyT, V> IntoIterator for &'a BinarySearchTree<K, V> {
    type Item = &'a K;

    type IntoIter = IntoIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}
