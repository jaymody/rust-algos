use std::{cmp::Ordering, fmt::Display, ops::Deref};

use crate::stack::{Stack, StackLinkedList};

use super::{KeyT, SymbolTable};

type Link<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K: KeyT, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    is_red: bool,
}

/// A red black binary search tree.
///
/// A red black BST is an implementation of a 2-3 such that the underlying
/// data structure is just a BST, with the insert and delete functions
/// doing the heavy lifting to maintain balance.
pub struct RedBlackBST<K: KeyT, V> {
    root: Link<K, V>,
    size: usize,
}

impl<K: KeyT, V> RedBlackBST<K, V> {
    pub fn new() -> Self {
        RedBlackBST {
            root: None,
            size: 0,
        }
    }

    /// Given a node n, with the right link is a red link and the left is a
    /// black, we want to rotate the structure such that the red link is on the
    /// left as such:
    ///
    /// ```text
    ///        n
    ///      /   \\
    ///           r
    ///          / \
    ///         m
    ///
    ///           r
    ///         // \
    ///        n
    ///      /   \
    ///           m
    /// ```
    ///
    /// Notice, we return r so that the link for the node pointing to n can be
    /// updated to r.
    fn rotate_left(mut n: Node<K, V>) -> Link<K, V> {
        let mut r = n.right.take().unwrap();

        r.is_red = n.is_red;
        n.is_red = true;

        n.right = r.left.take();
        r.left = Some(Box::new(n));

        Some(r)
    }

    /// Given a node n, with two consecutive left links that are red, we need to
    /// rotate the structure to the right:
    ///
    /// ```text
    ///          |
    ///          n
    ///        // \
    ///       m
    ///     // \
    ///     l   r
    ///
    ///          |
    ///          m
    ///        // \\
    ///       l    n
    ///           /
    ///          r
    /// ```
    ///
    /// Then, we run the flip colors function giving us a final result of:
    ///
    /// ```text
    ///          ||
    ///          m
    ///        /   \
    ///       l     n
    ///            /
    ///           r
    /// ```
    ///
    /// The incoming link is guaranteed to be black (so when we convert to red
    /// we aren't converting something that is already red) because rotations
    /// are run bottom up, so anything above us must be a valid red-black BST
    /// and a valid red-black BST cannot have two red links in a row.
    ///
    /// Finally, we return m so that the link for the node pointing to n can be
    /// update to r.
    fn rotate_right(mut n: Node<K, V>) -> Link<K, V> {
        let mut m = n.left.take().unwrap();
        n.left = m.right.take();
        n.is_red = true;
        m.right = Some(Box::new(n));
        Self::flip_colors(*m)
    }

    /// Given a node n, where both it's left and right link are red, we simply
    /// change the links to be black and update incoming link to be red:
    ///
    /// ```text
    ///       |
    ///       n
    ///     // \\
    ///    l     r
    ///
    ///       ||
    ///       n
    ///     /   \
    ///    l     r
    /// ```
    ///
    fn flip_colors(mut n: Node<K, V>) -> Link<K, V> {
        n.left.as_mut().unwrap().is_red = false;
        n.right.as_mut().unwrap().is_red = false;
        n.is_red = true;
        Some(Box::new(n))
    }

    fn insert(link: &mut Link<K, V>, node_to_insert: Node<K, V>) -> Link<K, V> {
        match link.take() {
            // we keep searching down the tree for a null link to place the
            // new node (or until we find a match in which case we just update)
            None => Some(Box::new(node_to_insert)),
            Some(mut node) => {
                // check if we need to go left, right, or we've hit a match
                match node_to_insert.key.cmp(&node.key) {
                    Ordering::Equal => node.val = node_to_insert.val,
                    Ordering::Less => node.left = Self::insert(&mut node.left, node_to_insert),
                    Ordering::Greater => node.right = Self::insert(&mut node.right, node_to_insert),
                }

                // If we've inserted a new node, our red black tree structure
                // invariants may have been violated in 3 possible ways:
                //
                //   1) there is a left and a right red link (in which case we flip the colors)
                //   2) there are two left red links in a row (in which case we rotate right and flip colors)
                //   3) there is a right red link (in which case we rotate left)
                //
                let some_and_red = |n: Option<&Box<Node<K, V>>>| n.map_or(false, |x| x.is_red);

                let left = (&node).left.as_ref();
                let right = (&node).right.as_ref();
                let left_left = left.map_or(None, |n| n.left.as_ref());

                let left_is_red = some_and_red(left);
                let right_is_red = some_and_red(right);
                let left_left_is_red = some_and_red(left_left);

                if left_is_red && right_is_red {
                    Self::flip_colors(*node)
                } else if left_is_red && left_left_is_red {
                    Self::rotate_right(*node)
                } else if right_is_red {
                    Self::rotate_left(*node)
                } else {
                    Some(node)
                }
            }
        }
    }

    fn search<'a>(link: &'a Link<K, V>, key: K) -> Option<&'a Node<K, V>> {
        match link {
            None => None,
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => Some(node.deref()),
                Ordering::Greater => Self::search(&node.right, key),
                Ordering::Less => Self::search(&node.left, key),
            },
        }
    }
}

impl<K: KeyT, V> SymbolTable<K, V> for RedBlackBST<K, V> {
    fn put(&mut self, key: K, val: V) -> Result<(), String> {
        let node_to_insert = Node {
            key: key,
            val: val,
            left: None,
            right: None,
            is_red: true, // new node must be a red link
        };

        let mut root = self.root.take();
        self.root = Self::insert(&mut root, node_to_insert);

        // root is kept black since it is technically not part of a 3-node
        self.root.as_mut().unwrap().is_red = false;

        self.size += 1;
        Ok(())
    }

    fn get(&self, key: K) -> Option<&V> {
        let node = Self::search(&self.root, key)?;
        Some(&node.val)
    }

    #[allow(unused_variables)]
    fn pop(&mut self, key: K) -> Option<V> {
        self.size -= 1;
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<K: KeyT + Display, V> Display for RedBlackBST<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /// implementation based on: https://stackoverflow.com/a/42449385/11070463
        fn get_suffix(s: &str, prefix: &str, is_red: bool) -> String {
            let mut s = s.to_string();
            if is_red {
                s = format!("\x1b[01;31m{}\x1b[00m", s);
            }
            format!("{}{}", prefix.clone(), s)
        }

        fn visit<K: KeyT + Display, V>(
            link: &Link<K, V>,
            prefix: String,
            output: &mut String,
            is_right: bool,
        ) {
            if let Some(node) = link {
                let out_str = format!(
                    "{}{}\n",
                    get_suffix(
                        if is_right { "|-- " } else { "\\-- " },
                        &prefix,
                        node.is_red,
                    ),
                    node.key
                );
                let new_prefix =
                    get_suffix(if is_right { "|   " } else { "    " }, &prefix, node.is_red);

                output.push_str(&out_str);
                visit(&node.right, new_prefix.clone(), output, true);
                visit(&node.left, new_prefix.clone(), output, false);
            }
        }

        let mut output = "".to_string();
        visit(&self.root, "".to_string(), &mut output, false);
        write!(f, "{}", output)
    }
}

pub struct IntoIter<'a, K: KeyT, V> {
    pub stack: StackLinkedList<&'a Node<K, V>>,
}

impl<'a, K: KeyT, V> IntoIter<'a, K, V> {
    pub fn new(tree: &'a RedBlackBST<K, V>) -> Self {
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

impl<'a, K: KeyT, V> IntoIterator for &'a RedBlackBST<K, V> {
    type Item = &'a K;

    type IntoIter = IntoIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut st = RedBlackBST::new();
        assert_eq!(st.is_empty(), true);
        assert_eq!(st.size(), 0);

        st.put("S", 1).unwrap();
        st.put("E", 2).unwrap();
        st.put("A", 3).unwrap();
        st.put("R", 4).unwrap();
        st.put("C", 5).unwrap();
        st.put("H", 6).unwrap();
        st.put("X", 7).unwrap();
        st.put("M", 8).unwrap();
        st.put("P", 9).unwrap();
        st.put("L", 10).unwrap();

        assert_eq!(st.size(), 10);
        assert_eq!(st.is_empty(), false);

        assert_eq!(st.get("S"), Some(&1));
        assert_eq!(st.get("E"), Some(&2));
        assert_eq!(st.get("A"), Some(&3));
        assert_eq!(st.get("R"), Some(&4));
        assert_eq!(st.get("C"), Some(&5));
        assert_eq!(st.get("H"), Some(&6));
        assert_eq!(st.get("X"), Some(&7));
        assert_eq!(st.get("M"), Some(&8));
        assert_eq!(st.get("P"), Some(&9));
        assert_eq!(st.get("L"), Some(&10));

        let mut iter = (&st).into_iter();
        assert_eq!(iter.next(), Some(&"A"));
        assert_eq!(iter.next(), Some(&"C"));
        assert_eq!(iter.next(), Some(&"E"));
        assert_eq!(iter.next(), Some(&"H"));
        assert_eq!(iter.next(), Some(&"L"));
        assert_eq!(iter.next(), Some(&"M"));
        assert_eq!(iter.next(), Some(&"P"));
        assert_eq!(iter.next(), Some(&"R"));
        assert_eq!(iter.next(), Some(&"S"));
        assert_eq!(iter.next(), Some(&"X"));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_example_2() {
        let mut st = RedBlackBST::new();
        assert_eq!(st.is_empty(), true);
        assert_eq!(st.size(), 0);

        st.put("A", 1).unwrap();
        st.put("C", 2).unwrap();
        st.put("E", 3).unwrap();
        st.put("H", 4).unwrap();
        st.put("L", 5).unwrap();
        st.put("M", 6).unwrap();
        st.put("P", 7).unwrap();
        st.put("R", 8).unwrap();
        st.put("S", 9).unwrap();
        st.put("X", 10).unwrap();

        assert_eq!(st.size(), 10);
        assert_eq!(st.is_empty(), false);

        assert_eq!(st.get("A"), Some(&1));
        assert_eq!(st.get("C"), Some(&2));
        assert_eq!(st.get("E"), Some(&3));
        assert_eq!(st.get("H"), Some(&4));
        assert_eq!(st.get("L"), Some(&5));
        assert_eq!(st.get("M"), Some(&6));
        assert_eq!(st.get("P"), Some(&7));
        assert_eq!(st.get("R"), Some(&8));
        assert_eq!(st.get("S"), Some(&9));
        assert_eq!(st.get("X"), Some(&10));

        let mut iter = (&st).into_iter();
        assert_eq!(iter.next(), Some(&"A"));
        assert_eq!(iter.next(), Some(&"C"));
        assert_eq!(iter.next(), Some(&"E"));
        assert_eq!(iter.next(), Some(&"H"));
        assert_eq!(iter.next(), Some(&"L"));
        assert_eq!(iter.next(), Some(&"M"));
        assert_eq!(iter.next(), Some(&"P"));
        assert_eq!(iter.next(), Some(&"R"));
        assert_eq!(iter.next(), Some(&"S"));
        assert_eq!(iter.next(), Some(&"X"));
        assert_eq!(iter.next(), None);
    }
}
