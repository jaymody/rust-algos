use super::PriorityQueue;

/// A MinHeap (binary heap) implementation for a priority queue, using a fixed
/// length array.
///
/// A min heap is a binary tree structure with the constraints:
///
///   a) For any given node, all of it's descendants must be <= to it.
///   b) The tree is complete (all nodes except at the last level are full, that
///      is have two children, and the nodes in the last level are as far left
///      as possible).
///
/// Because of property a), the root of a min heap will always be the minimum
/// value of the entire tree. Because of property b), we can represent the tree
/// as contiguous values in an array in level-order (left-to-right, top-down).
///
/// For example, the following tree:
///
/// ```text
///            1
///          /   \
///         5     2
///        / \   / \
///       6   7 4   3
///      / \
///     9   8
/// ```
///
/// would be represented by the array:
///
/// ```text
/// arr = [1, 5, 2, 6, 7, 4, 3, 9, 8]
/// ```
///
/// A nice property here is that if we index our array starting from 1, for any
/// given node with index k:
///
///   1) It's parent is found at `k/2` (integer division, so if odd, remove the
///        decimal)
///   2) It's left child is found at `2k`
///   3) It's right child is found at `2k+1`
///
/// For example, for the node with value 6, with index k=4:
///
///   1) parent      = `arr[k/2]`   = `arr[2]`  = 5
///   2) left child  = `arr[2k]`    = `arr[8]`  = 9
///   2) right child = `arr[2k+1]`  = `arr[9]`  = 8
///
/// Another nice property that follows from the above, is that if there is
/// a node that is out of place, i.e. breaks constraint a), then we can fix
/// our tree in O(log n) time using a "sink" operation to move the node down
/// the tree until it is in a correct position, or a "swim" operation to move
/// the node up the tree until it is in a correct position.
///
/// See: https://algs4.cs.princeton.edu/24pq/
pub struct PriorityQueueBinaryHeap<T: Ord, const CAPACITY: usize> {
    arr: [Option<T>; CAPACITY],
    size: usize,
}

impl<T: Ord, const CAPACITY: usize> PriorityQueueBinaryHeap<T, CAPACITY> {
    const INIT: Option<T> = None;
    pub fn new() -> Self {
        PriorityQueueBinaryHeap {
            arr: [Self::INIT; CAPACITY],
            size: 0,
        }
    }

    /// Sinks node at position i down to it's correct position. If the given
    /// node is less than both it's children, then we are done (the node is
    /// already in the correct spot). If it's greater than one or both children,
    /// we swap it with the lesser child and repeat the process for the lesser
    /// child. Sink operation is O(log n).
    fn sink(&mut self, i: usize) {
        let l = 2 * i;
        let r = 2 * i + 1;

        if l < self.size {
            let min_child_i = if r < self.size && self.arr[r] < self.arr[l] {
                r
            } else {
                l
            };

            if self.arr[i] > self.arr[min_child_i] {
                self.arr.swap(i, min_child_i);
                self.sink(min_child_i);
            }
        }
    }

    /// Swim node at position i up to it's correct position. If the given
    /// node is greater than it's parent, then we are done (the node is already
    /// in the correct spot). Else, we swap with the parent and repeat the
    /// process with the parents position. Swim operation is O(log n).
    fn swim(&mut self, i: usize) {
        if i > 0 {
            let p = i / 2;
            if self.arr[i] < self.arr[p] {
                self.arr.swap(i, p);
                self.swim(p);
            }
        }
    }
}

impl<T: Ord, const CAPACITY: usize> PriorityQueue<T> for PriorityQueueBinaryHeap<T, CAPACITY> {
    /// Push an item to the queue.
    ///
    /// ### Implementation
    /// We add the item to the end of the array, and then swim it up to it's
    /// correct position in the heap in O(log n) time.
    fn push(&mut self, item: T) -> Result<(), String> {
        if self.size >= CAPACITY {
            return Err("capacity full".to_string());
        }
        self.arr[self.size] = Some(item);
        self.swim(self.size);
        self.size += 1;
        Ok(())
    }

    /// Pop an item from the queue (return None if the queue is empty).
    ///
    /// ### Implementation
    /// The minimum entry is the root of our heap (i.e. the first entry in
    /// the array). To maintain the array contiguous, we swap the first and last
    /// entry, and then remove and return the last entry (which is our min val).
    /// However, now our first entry is no longer in it's correct position, so
    /// we use the sink operation to move it down to it's correct position in
    /// O(log n) time.
    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let min = self.arr[0].take();
        self.arr.swap(0, self.size - 1);
        self.size -= 1;
        self.sink(0);
        min
    }

    /// Peek at the next item in the queue (return None if the queue is empty).
    ///
    /// ### Implementation
    /// The minimum entry is the root of our heap (i.e. the first entry in
    /// the array), so we simply return a reference of arr[0].
    fn peek(&self) -> Option<&T> {
        self.arr[0].as_ref()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
