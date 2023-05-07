pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    // heapify
    for i in (0..arr.len()).rev() {
        sink(arr, i);
    }

    // sort down
    for i in (0..arr.len()).rev() {
        arr.swap(0, i);
        sink(&mut arr[..i], 0);
    }
    arr.reverse()
}

fn sink<T: Ord>(arr: &mut [T], i: usize) {
    let l = 2 * i;
    let r = 2 * i + 1;

    if l < arr.len() {
        let min_child_i = if r < arr.len() && arr[r] < arr[l] {
            r
        } else {
            l
        };

        if arr[i] > arr[min_child_i] {
            arr.swap(i, min_child_i);
            sink(arr, min_child_i);
        }
    }
}
