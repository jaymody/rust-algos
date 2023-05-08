pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let aux = &mut arr.to_vec()[..];
    sort(arr, aux);
}

fn sort<T: Ord + Clone>(arr: &mut [T], aux: &mut [T]) {
    if arr.len() > 1 {
        let m = arr.len() / 2;
        sort(&mut arr[..m], &mut aux[..m]);
        sort(&mut arr[m..], &mut aux[m..]);
        merge(arr, &aux[..m], &aux[m..]);
        aux.clone_from_slice(arr);
    }
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    while i + j < arr.len() {
        if i >= left.len() {
            arr[i + j] = right[j].clone();
            j += 1;
        } else if j >= right.len() {
            arr[i + j] = left[i].clone();
            i += 1;
        } else if left[i] < right[j] {
            arr[i + j] = left[i].clone();
            i += 1;
        } else {
            arr[i + j] = right[j].clone();
            j += 1;
        }
    }
}
