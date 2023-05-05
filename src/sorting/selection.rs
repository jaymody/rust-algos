pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut min_idx = i - 1;
        for j in i..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i - 1, min_idx);
    }
}
