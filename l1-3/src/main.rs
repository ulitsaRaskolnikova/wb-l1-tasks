use std::sync::mpsc;
use std::thread;

fn compute_sum_of_squares() {
    const N: usize = 10;
    let mut numbers: [u32; N] = [0; N];

    for i in 0..N {
        numbers[i] = (i + 1) as u32;
    }

    let (tx, rx) = mpsc::channel();

    for num in numbers {
        let tx = tx.clone();
        thread::spawn(move || {
            let square = num * num;
            println!("{}^2 = {}", num, square);
            tx.send(square).unwrap();
        });
    }

    let mut sum = 0;
    for _ in 0..N {
        sum += rx.recv().unwrap();
    }

    println!("Sum is {}", sum);
}

fn main() {
    compute_sum_of_squares();
}