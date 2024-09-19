fn main() {
    let mut numbers = vec![10, 20, 30, 40, 50];

    println!("Before removal: {:?}", numbers);

    let index_to_remove = 2;

    if index_to_remove < numbers.len() {
        numbers.remove(index_to_remove);
        println!("After removal: {:?}", numbers);
    } else {
        println!("Index out of bounds!");
    }
}
