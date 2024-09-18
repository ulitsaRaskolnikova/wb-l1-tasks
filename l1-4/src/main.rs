use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::io;

fn main() {
    let mut workers = String::new();

    io::stdin()
        .read_line(&mut workers)
        .expect("Failed to read line");

    let workers: u32 = workers.trim().parse().unwrap();

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut handles = vec![];
    for i in 0..workers {
        let receiver = Arc::clone(&receiver);
        let handle = thread::spawn(move || {
            consume_messages(i, receiver);
        });
        handles.push(handle);
    }
    let mut i = 0;
    loop {
        let message = format!("Message {}", i);
        sender.send(message).unwrap();
        i += 1;
        thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn consume_messages(id: u32, receiver: Arc<Mutex<mpsc::Receiver<String>>>) {
    // слушаем главный канал, пока что-то получаем от него
    loop {
        let message = receiver.lock().unwrap().recv();
        match message {
            Ok(message) => {
                println!("Consumer {} received: {}", id, message); 
                thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(_) => break,
        }
    }
}