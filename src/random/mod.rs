use rand::Rng;

pub fn shuffle_arr<T: Ord>(arr: &mut [T]) {
    // fisher yates algorithm to shuffle an array
    for i in (1..arr.len()).rev() {
        let j = rand::thread_rng().gen_range(0..=i);
        arr.swap(i, j);
    }
}
