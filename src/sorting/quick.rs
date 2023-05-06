use rand::seq::SliceRandom;

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    arr.shuffle(&mut rand::thread_rng());
    sort(arr);
}

fn sort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        let k = partition(arr);
        sort(&mut arr[..k]);
        sort(&mut arr[k + 1..]);
    }
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let k = arr.len() - 1;

    let mut i = 0;
    for j in 0..k {
        if arr[j] < arr[k] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, k);
    i
}
