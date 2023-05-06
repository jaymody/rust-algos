pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i <= j {
        let m = (i + j) / 2;
        if arr[m] == target {
            return Some(m);
        } else if arr[m] > target {
            if m == 0 {
                return None;
            }
            j = m - 1;
        } else {
            i = m + 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(binary_search(&arr, 3), Some(2));
        assert_eq!(binary_search(&arr, 10), Some(9));
        assert_eq!(binary_search(&arr, 1), Some(0));
        assert_eq!(binary_search(&arr, 6), Some(5));
        assert_eq!(binary_search(&arr, 11), None);
        assert_eq!(binary_search(&arr, -1), None);
    }
}
