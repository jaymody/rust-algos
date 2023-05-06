pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    for n in 0..=calc_max_n(arr.len()) {
        h_sort(arr, calc_h(n))
    }
}

fn h_sort<T: Ord>(arr: &mut [T], h: usize) {
    for offset in 0..h {
        for i in (offset..arr.len()).step_by(h) {
            let mut j = i;
            while j >= h && arr[j] < arr[j - h] {
                arr.swap(j, j - h);
                j -= h;
            }
        }
    }
}

fn calc_h(n: usize) -> usize {
    /*
    The series for values that h should take on is:
        1, 4, 13, 40, 121 ...
    Which more concretely is described with the recurrence relation:
        a(0) = 1
        a(n) = 3 * a(n-1) + 1
    This can be shown to be equivalent to the geometric series
        a(n) = sum(3^k for k in 0..=n)
    Geometric series have closed form solution, giving us:
        a(n) = (1 - r^(n+1)) / (1 - r)
        a(n) = (1 - 3^(n+1)) / (1 - 3)
        a(n) = (1 - 3^(n+1)) / -2
    So for some given n, our equation for h is:
        h = (1 - 3^(n+1)) / -2
    */
    let h = (1 - (3 as i32).pow(n as u32 + 1)) / -2;
    debug_assert!(h > 0);
    h as usize
}

fn calc_max_n(arr_len: usize) -> usize {
    /*
    Given the size of the input array, we want to figure out for the max value
    for n that satisfies a(n) < arr.len() (i.e. what is the maximum h value we
    start sorting with).

    From calc_h, we saw that the formula to compute h given n is:
        h = (1 - 3^(n+1)) / -2

    We can rearrange the equation to solve instead for n:
        h = (1 - 3^(n+1)) / -2
        -2h = 1 - 3^(n+1)
        3^(n+1) = 2h + 1
        log(3^(n+1)) = log(2h + 1)
        (n + 1) * log(3) = log(2h + 1)
        n = log(2h + 1) / log(3) - 1

    So, we set h = arr.len()-1, we can get the value of n using the above. Of
    course, it's likely the size of our array minus one is not a value in our
    series, and so we'll get a non-integer value for n. In that case, we simply
    take the floor to give us the n corresponding to the maximum possible h
    in our series that is still less than the array size.
    */
    if arr_len < 2 {
        return 0;
    }
    let h = arr_len - 1;
    ((2.0 * h as f64 + 1.0).log2() / (3.0 as f64).log2() - 1.0).floor() as usize
}

#[cfg(test)]
mod tests {
    use super::{calc_h, calc_max_n};

    #[test]
    fn test_calc_h_and_calc_max_n() {
        // test calc_h
        let h_vals: Vec<usize> = vec![1, 4, 13, 40, 121, 364];
        let h_calc: Vec<usize> = (0..h_vals.len()).map(calc_h).collect();
        assert_eq!(h_calc, h_vals);

        // test calc_max_n
        // arr_len = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        //   max_h = 0, 0, 1, 1, 1, 4, 4, 4, 4, 4,  4,  4,  4,  4, 13, 13
        //       n = 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,  1,  1,  1,  1,  2,  2
        let n_vals: Vec<usize> = vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2];
        let n_calc: Vec<usize> = (0..n_vals.len()).map(calc_max_n).collect();
        assert_eq!(n_calc, n_vals);
    }
}
