use std::any::type_name;

fn print_type_of<T: 'static>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn main() {
    let x = 42; // i32
    let y = 3.14; // f64
    let z = "Hello, world!"; // &str

    print_type_of(&x);
    print_type_of(&y);
    print_type_of(&z);
}
