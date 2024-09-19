use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx2) = mpsc::channel();
    let (tx2, rx3) = mpsc::channel();

    let mut handles = Vec::new();

    let handle = thread::spawn(move || loop {
        let num = rx2.recv();
        match num {
            Ok(num) => tx2.send(num * num).unwrap(),
            _ => break,
        }
    });

    handles.push(handle);

    let handle = thread::spawn(move || loop {
        let square = rx3.recv();
        match square {
            Ok(square) => println!("{}", square),
            _ => break,
        }
    });

    handles.push(handle);

    const N: usize = 10;
    let mut numbers: [u32; N] = [0; N];

    for i in 0..N {
        numbers[i] = (i + 1) as u32;
    }

    for i in 0..N {
        tx1.send(numbers[i]).unwrap();
    }
    drop(tx1);

    for handle in handles  {
        handle.join().unwrap();
    }
}