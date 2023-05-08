use std::ops::Deref;

use super::{KeyT, SymbolTable};

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
        fn visit<K: KeyT, V>(mut link: Link<K, V>, key: K) -> (Link<K, V>, Option<Node<K, V>>) {
            match link.take() {
                None => (None, None),
                Some(mut node) => {
                    if key == node.key {
                        if node.right.is_none() {
                            (node.left.take(), Some(*node))
                        } else {
                            let successor;
                            (node.right, successor) = find_successor(node.right);
                            (successor, Some(*node))
                        }
                    } else if key > node.key {
                        let deleted_node;
                        (node.right, deleted_node) = visit(node.right, key);
                        (Some(node), deleted_node)
                    } else {
                        let deleted_node;
                        (node.left, deleted_node) = visit(node.left, key);
                        (Some(node), deleted_node)
                    }
                }
            }
        }

        fn find_successor<K: Ord + Copy + Clone, V>(
            mut link: Link<K, V>,
        ) -> (Link<K, V>, Link<K, V>) {
            match link.take() {
                None => (None, None),
                Some(mut node) => {
                    let successor;
                    (node.left, successor) = find_successor(node.left);
                    (Some(node), successor)
                }
            }
        }

        let deleted_node;
        let root = self.root.take();
        (self.root, deleted_node) = visit(root, key);
        self.size -= 1;
        deleted_node
    }
}

impl<K: KeyT, V> SymbolTable<K, V> for BinarySearchTree<K, V> {
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

pub struct IntoIter<'a, K: KeyT, V> {
    tree: &'a BinarySearchTree<K, V>,
}

impl<'a, K: KeyT, V> Iterator for IntoIter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, K: KeyT, V> IntoIterator for &'a BinarySearchTree<K, V> {
    type Item = &'a K;

    type IntoIter = IntoIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
