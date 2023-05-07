pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }

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

pub fn binary_search_insert_index<T: Ord>(arr: &[T], target: T) -> usize {
    if arr.len() == 0 {
        return 0;
    }

    let mut i = 0;
    let mut j = arr.len() - 1;
    while i <= j {
        let m = (i + j) / 2;
        if arr[m] == target {
            return m;
        } else if arr[m] > target {
            if m == 0 {
                return 0;
            }
            j = m - 1;
        } else {
            i = m + 1;
        }
    }
    i
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

    #[test]
    fn test_binary_search_insert_index() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(binary_search_insert_index(&arr, 3), 2);
        assert_eq!(binary_search_insert_index(&arr, 10), 9);
        assert_eq!(binary_search_insert_index(&arr, 1), 0);
        assert_eq!(binary_search_insert_index(&arr, 6), 5);
        assert_eq!(binary_search_insert_index(&arr, 11), 10);
        assert_eq!(binary_search_insert_index(&arr, -1), 0);

        let arr = [1, 2, 2, 4, 4, 6, 9, 9, 9, 10];
        assert_eq!(binary_search_insert_index(&arr, 11), 10);
        assert_eq!(binary_search_insert_index(&arr, -1), 0);
        assert_eq!(binary_search_insert_index(&arr, 0), 0);
        assert_eq!(binary_search_insert_index(&arr, 1), 0);
        assert_eq!(binary_search_insert_index(&arr, 3), 3);
        assert_eq!(binary_search_insert_index(&arr, 5), 5);
        assert_eq!(binary_search_insert_index(&arr, 6), 5);
        assert_eq!(binary_search_insert_index(&arr, 7), 6);
        assert_eq!(binary_search_insert_index(&arr, 8), 6);

        let arr = [];
        assert_eq!(binary_search_insert_index(&arr, 10), 0);
        assert_eq!(binary_search_insert_index(&arr, -10), 0);

        let arr = [1];
        assert_eq!(binary_search_insert_index(&arr, 0), 0);
        assert_eq!(binary_search_insert_index(&arr, 1), 0);
        assert_eq!(binary_search_insert_index(&arr, 2), 1);

        let arr = [1, 3];
        assert_eq!(binary_search_insert_index(&arr, 0), 0);
        assert_eq!(binary_search_insert_index(&arr, 1), 0);
        assert_eq!(binary_search_insert_index(&arr, 2), 1);
        assert_eq!(binary_search_insert_index(&arr, 3), 1);
        assert_eq!(binary_search_insert_index(&arr, 4), 2);
    }
}
