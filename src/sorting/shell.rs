pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    let mut h = calc_max_h(arr.len());
    while h > 0 {
        h_sort(arr, h);
        h /= 3;
    }
}

fn h_sort<T: Ord>(arr: &mut [T], h: usize) {
    for i in h..arr.len() {
        let mut j = i;
        while j >= h && arr[j] < arr[j - h] {
            arr.swap(j, j - h);
            j -= h;
        }
    }
}

fn calc_max_h(arr_len: usize) -> usize {
    /*
    Given the size of the input array, we want to find what is the maximum h
    value in our sequence that is less than the array size.

    Recall, the sequence for values that h should take on is:

        1, 4, 13, 40, 121 ...

    Which more concretely is described with the recurrence relation:

        a(0) = 1
        a(n) = 3 * a(n-1) + 1

    With this, in theory, we could just compute the value for h with:

        let h = 1;
        while h < n/3 { h = 3*h + 1 }

    Technically, there does exist a closed form solution to calculate h instead
    of computing it.

    Is it worth even optimizing this further? No, the time complexity for the
    above code is simply log3(n), which is very fast and given our sorting
    algorithm is way more costly, at scale this contributes a small, small,
    VERY small amount to the run time.

    Am I going to derive the closed form solution anyways? Yep.

    So first, let's just inspect our series a bit more:

        1, 4, 13, 40, 121 ...

    If we take the difference between adjacent entries in the sequence, we get:

        a(1) - a(0)  = 4 - 1    = 3
        a(2) - a(1)  = 13 - 4   = 9
        a(3) - a(2)  = 40 - 13  = 27
        a(4) - a(3)  = 121 - 40 = 81

    Wait a minute, that's just powers of 3 (3^1, 3^2, 3^3, etc ...). So
    another way to look at our sequence is:

        a(0) = 1
        a(1) = 4    = a(0)+3    = a(0)+3^1  = a(0) + 3^1
        a(2) = 13   = a(1)+9    = a(1)+3^2  = a(0) + 3^1 + 3^2
        a(3) = 40   = a(2)+27   = a(2)+3^3  = a(0) + 3^1 + 3^2 + 3^3
        a(4) = 121  = a(3)+81   = a(3)+3^4  = a(0) + 3^1 + 3^2 + 3^3 + 3^4

    More generally, we can describe a(n) as:

        a(n) = a(0) + sum(3^k for k in 1..=n)
        a(n) = 1 + sum(3^k for k in 1..=n)
        a(n) = sum(3^k for k in 0..=n)

    This is just a geometric series with r = 3 and a = 1, for which there is a
    closed form solution:

        a(n) = (1 - r^(n+1)) / (1 - r)
        a(n) = (1 - 3^(n+1)) / (1 - 3)
        a(n) = (1 - 3^(n+1)) / -2

    So for some given n, our equation for h is:

        h = (1 - 3^(n+1)) / -2

    We can rearrange the equation to solve n as well:

        h = (1 - 3^(n+1)) / -2
        -2h = 1 - 3^(n+1)
        3^(n+1) = 2h + 1
        log(3^(n+1)) = log(2h + 1)
        (n + 1) * log(3) = log(2h + 1)
        n = log(2h + 1) / log(3) - 1

    So, to get the maximum value of h that is less than the array size, we first
    need to find the value of n corresponding to the array size minus one. That
    is, we set h = arr_len - 1 in the above equation (we subtract one because we
    want a value for h < the array size and not <=). Likely, this value for n
    will not be an integer (if it were, that would mean that arr_len - 1 happens
    to be one of the values in our sequence). Since we are looking for an h
    value less than the array size, we simply take the floor of our
    calculated n value.

    Finally, we return the h value corresponding to the floor of n.

    Also, to handle the edge case where arr_len < 2 (our functions are not
    defined at those points), we simply return 0, since an array with length
    0 or 1 are already sorted.

    Was all this work worth it to save a little bit of computation? Nope. But
    I did it to (dis)honor of Knuth.

        "Premature optimization is the root of all evil" - Donald Knuth
    */
    if arr_len < 2 {
        return 0;
    }

    let n = ((2.0 * (arr_len - 1) as f64 + 1.0).log2() / 3.0_f64.log2() - 1.0).floor() as usize;
    let h = (1 - 3_i32.pow(n as u32 + 1)) / -2;

    h as usize
}

#[cfg(test)]
mod tests {
    use super::calc_max_h;

    #[test]
    fn test_calc_max_h() {
        // test calc_max_n
        // arr_len = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        //   max_h = 0, 0, 1, 1, 1, 4, 4, 4, 4, 4,  4,  4,  4,  4, 13, 13
        let vals: Vec<usize> = vec![0, 0, 1, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 13, 13];
        let calc: Vec<usize> = (0..vals.len()).map(calc_max_h).collect();
        assert_eq!(calc, vals);
    }
}
