mod bubble;
mod heap;
mod insertion;
mod merge;
mod quick;
mod selection;
mod shell;

pub use bubble::bubble_sort;
pub use heap::heap_sort;
pub use insertion::insertion_sort;
pub use merge::merge_sort;
pub use quick::quick_sort;
pub use selection::selection_sort;
pub use shell::shell_sort;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn test_sort_fn<F>(sort_fn: F)
    where
        F: Fn(&mut [i32]),
    {
        let test_example = |input: &mut [i32], output: &[i32]| {
            sort_fn(input);
            assert_eq!(input, output);
        };

        test_example(&mut [], &[]);

        test_example(&mut [1], &[1]);

        test_example(&mut [2, 1], &[1, 2]);

        test_example(&mut [1, 2], &[1, 2]);

        test_example(&mut [0, 0, 0, 0, 0, 0], &[0, 0, 0, 0, 0, 0]);

        test_example(&mut [1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]);

        test_example(&mut [5, 4, 3, 2, 1], &[1, 2, 3, 4, 5]);

        test_example(
            &mut [1, 1, 5, 1, 6, 7, -3, 0, 0, 9],
            &[-3, 0, 0, 1, 1, 1, 5, 6, 7, 9],
        );

        const SIZE: usize = 20;
        let mut arr: [i32; SIZE] = std::array::from_fn(|_| rand::thread_rng().gen_range(-50..=50));
        let mut ans: [i32; SIZE] = [0; SIZE];
        ans.copy_from_slice(&arr);
        ans.sort();
        test_example(&mut arr, &ans)
    }

    #[test]
    fn test_bubble_sort() {
        test_sort_fn(bubble_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_sort_fn(insertion_sort);
    }

    #[test]
    fn test_selection_sort() {
        test_sort_fn(selection_sort);
    }

    #[test]
    fn test_shell_sort() {
        test_sort_fn(shell_sort);
    }

    #[test]
    fn test_quick_sort() {
        test_sort_fn(quick_sort);
    }

    #[test]
    fn test_merge_sort() {
        test_sort_fn(merge_sort);
    }

    #[test]
    fn test_heap_sort() {
        test_sort_fn(heap_sort);
    }
}
