pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    loop {
        let mut sorted = true;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}
