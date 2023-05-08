use std::ops::Deref;

use super::SymbolTable;

type Link<K, V> = Option<Box<Node<K, V>>>;

struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Node {
            key: key,
            val: val,
            left: None,
            right: None,
        }
    }
}

pub struct BinarySearchTree<K: Ord + Clone + Copy, V> {
    root: Link<K, V>,
    size: usize,
}

impl<K: Ord + Clone + Copy, V> BinarySearchTree<K, V> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    fn insert(&mut self, node_to_insert: Node<K, V>) {
        let mut root = self.root.take();
        self.root = self._insert(&mut root, node_to_insert);
        self.size += 1;
    }

    fn _insert(&mut self, link: &mut Link<K, V>, node_to_insert: Node<K, V>) -> Link<K, V> {
        match link.take() {
            None => Some(Box::new(node_to_insert)),
            Some(mut node) => {
                if node_to_insert.key == node.key {
                    node.val = node_to_insert.val;
                } else if node_to_insert.key > node.key {
                    node.right = self._insert(&mut node.right, node_to_insert);
                } else {
                    node.left = self._insert(&mut node.left, node_to_insert);
                }
                Some(node)
            }
        }
    }

    fn search(&self, key: K) -> Option<&Node<K, V>> {
        self._search(&self.root, key)
    }

    fn _search<'a>(&self, link: &'a Link<K, V>, key: K) -> Option<&'a Node<K, V>> {
        match link {
            None => None,
            Some(node) => {
                if key == node.key {
                    Some(node.deref())
                } else if key > node.key {
                    self._search(&node.right, key)
                } else {
                    self._search(&node.left, key)
                }
            }
        }
    }

    fn delete(&mut self, key: K) -> Option<Node<K, V>> {
        let deleted_node;
        let root = self.root.take();
        (self.root, deleted_node) = self._delete(root, key);
        self.size -= 1;
        deleted_node
    }

    fn _delete(&mut self, mut link: Link<K, V>, key: K) -> (Link<K, V>, Option<Node<K, V>>) {
        match link.take() {
            None => (None, None),
            Some(mut node) => {
                if key == node.key {
                    if node.right.is_none() {
                        (node.left.take(), Some(*node))
                    } else {
                        let successor;
                        (node.right, successor) = self.find_successor(node.right);
                        (successor, Some(*node))
                    }
                } else if key > node.key {
                    let deleted_node;
                    (node.right, deleted_node) = self._delete(node.right, key);
                    (Some(node), deleted_node)
                } else {
                    let deleted_node;
                    (node.left, deleted_node) = self._delete(node.left, key);
                    (Some(node), deleted_node)
                }
            }
        }
    }

    fn find_successor(&mut self, mut link: Link<K, V>) -> (Link<K, V>, Link<K, V>) {
        match link.take() {
            None => (None, None),
            Some(mut node) => {
                let successor;
                (node.left, successor) = self.find_successor(node.left);
                (Some(node), successor)
            }
        }
    }
}

impl<K: Ord + Clone + Copy, V> SymbolTable<K, V> for BinarySearchTree<K, V> {
    fn put(&mut self, key: K, val: V) -> Result<(), String> {
        self.insert(Node::new(key, val));
        Ok(())
    }

    fn get(&self, key: K) -> Option<&V> {
        Some(&self.search(key)?.val)
    }

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

pub struct IntoIter<'a, K: Ord + Clone + Copy, V> {
    tree: &'a BinarySearchTree<K, V>,
}

impl<'a, K: Ord + Clone + Copy, V> Iterator for IntoIter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, K: Ord + Clone + Copy, V> IntoIterator for &'a BinarySearchTree<K, V> {
    type Item = &'a K;

    type IntoIter = IntoIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
