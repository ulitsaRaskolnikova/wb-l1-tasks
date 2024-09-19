fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);

    let mut store_index = 0;

    for i in 0..arr.len() - 1 {
        if arr[i] <= arr[arr.len() - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }

    arr.swap(store_index, arr.len() - 1);
    store_index
}

fn main() {
    let mut arr = [3, 6, 8, 10, 1, 2, 1, 100];
    println!("Before sorting: {:?}", arr);

    quicksort(&mut arr);
    println!("After sorting: {:?}", arr);
}
