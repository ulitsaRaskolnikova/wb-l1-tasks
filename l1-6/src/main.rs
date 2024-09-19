use std::sync::mpsc;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        let message = rx.recv();
        match message {
            Ok(message) => println!("{}", message),
            _ => break,
        }
    });

    let seconds = Duration::from_secs(5);

    let mut i = 0;

    let start = SystemTime::now();

    loop {
        let val = format!("Message {}", i);

        tx.send(val).unwrap();
        i += 1;

        match start.elapsed() {
            Ok(elapsed) if elapsed > seconds => {
                return;
            }
            _ => (),
        }
    }
}