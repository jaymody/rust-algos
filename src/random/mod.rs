use rand::Rng;

pub fn fisher_yates_shuffle<T: Ord>(arr: &mut [T]) {
    for i in (1..arr.len()).rev() {
        let j = rand::thread_rng().gen_range(0..=i);
        arr.swap(i, j);
    }
}
