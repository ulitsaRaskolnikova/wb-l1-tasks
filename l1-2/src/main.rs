use std::thread;

fn compute_squares() {
    const N: usize = 10;
    let mut numbers: [u32; N] = [0; N];

    for i in 0..N {
        numbers[i] = (i + 1) as u32;
    }

    let mut handles = Vec::new();

    for num in numbers {
        let handle = thread::spawn(move || {
            let square = num * num;
            println!("{}^2 = {}", num, square);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    compute_squares();
}