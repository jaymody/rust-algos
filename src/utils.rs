pub fn insert_and_shift<T>(arr: &mut [T], item: T, i: usize) -> T {
    let mut prev = item;
    for j in i..arr.len() {
        prev = std::mem::replace(&mut arr[j], prev);
    }
    return prev;
}
