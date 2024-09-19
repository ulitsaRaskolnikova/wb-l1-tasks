fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() as isize - 1;

    while left <= right {
        let mid = (left + right) / 2;
        let mid_value = arr[mid as usize];

        if mid_value == target {
            return Some(mid as usize);
        } else if mid_value < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    let sorted_arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 101];

    let target = 100;

    match binary_search(&sorted_arr, target) {
        Some(index) => println!("Элемент найден на индексе: {}", index),
        None => println!("Элемент не найден в массиве"),
    }
}
