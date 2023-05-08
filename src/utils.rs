/// Inserts `item` at index `i` in the array `arr`, shifting the array to
/// the right by one to make space for the item. As a result, the last item
/// in the array no longer has a spot to move to, so it is returned.
///
/// ### Example
/// ```
/// let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let last = insert_and_shift(&mut arr, 100, 3);
/// assert_eq!(arr, [0, 1, 2, 100, 3, 4, 5, 6, 7, 8]);
/// assert_eq!(last, 9);
/// ```
pub fn insert_and_shift<T>(arr: &mut [T], item: T, i: usize) -> T {
    let mut prev = item;
    for j in i..arr.len() {
        prev = std::mem::replace(&mut arr[j], prev);
    }
    return prev;
}

/// Pops the entry at index `i` in the array, and shifts the array to the
/// left by one to fill it's place. As a result, the last item in the array
/// will be empty and requires a value, so it is filled with `last`.
///
/// ### Example
/// ```
/// let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let deleted = pop_and_shift(&mut arr, 100, 3);
/// assert_eq!(arr, [0, 1, 2, 4, 5, 6, 7, 8, 9, 100]);
/// assert_eq!(deleted, 3);
/// ```
pub fn pop_and_shift<T>(arr: &mut [T], last: T, i: usize) -> T {
    let len = arr.len();
    arr[i..len].rotate_left(1);
    std::mem::replace(&mut arr[len - 1], last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_shift() {
        let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(insert_and_shift(&mut arr, 0, 1), 9);
        assert_eq!(arr, [0, 0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(insert_and_shift(&mut arr, 10, 0), 8);
        assert_eq!(arr, [10, 0, 0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(insert_and_shift(&mut arr, -1, 9), 7);
        assert_eq!(arr, [10, 0, 0, 1, 2, 3, 4, 5, 6, -1]);
        assert_eq!(insert_and_shift(&mut arr, -2, 8), -1);
        assert_eq!(arr, [10, 0, 0, 1, 2, 3, 4, 5, -2, 6]);
    }

    #[test]
    fn test_pop_and_shift() {
        let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(pop_and_shift(&mut arr, -1, 3), 3);
        assert_eq!(arr, [0, 1, 2, 4, 5, 6, 7, 8, 9, -1]);
        assert_eq!(pop_and_shift(&mut arr, -2, 9), -1);
        assert_eq!(arr, [0, 1, 2, 4, 5, 6, 7, 8, 9, -2]);
        assert_eq!(pop_and_shift(&mut arr, 100, 0), 0);
        assert_eq!(arr, [1, 2, 4, 5, 6, 7, 8, 9, -2, 100]);
        assert_eq!(pop_and_shift(&mut arr, 200, 1), 2);
        assert_eq!(arr, [1, 4, 5, 6, 7, 8, 9, -2, 100, 200]);
    }
}
