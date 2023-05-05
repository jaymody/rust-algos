pub mod bubble;
pub mod insert;
pub mod selection;

pub use bubble::bubble_sort;
pub use insert::insert_sort;
pub use selection::selection_sort;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sort_fn<F>(sort_fn: F)
    where
        F: Fn(&mut [i32]) -> (),
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
    }

    #[test]
    fn test_bubble_sort() {
        test_sort_fn(bubble_sort);
    }

    #[test]
    fn test_insert_sort() {
        test_sort_fn(insert_sort);
    }

    #[test]
    fn test_selection_sort() {
        test_sort_fn(selection_sort);
    }
}
