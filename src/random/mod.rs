use rand::Rng;

/// Fisher yates algorithm to shuffle an array in-place in O(1) time.
pub fn shuffle_arr<T: Ord>(arr: &mut [T]) {
    for i in (1..arr.len()).rev() {
        let j = rand::thread_rng().gen_range(0..=i);
        arr.swap(i, j);
    }
}
