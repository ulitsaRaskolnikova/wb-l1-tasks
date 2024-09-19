use std::sync::{Arc, Mutex};
use std::thread;

struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

impl SharedCounter {
    fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }

    fn inc(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }

    fn get(&self) -> i32 {
        let num = self.value.lock().unwrap();
        *num
    }

    fn clone(&self) -> SharedCounter {
        SharedCounter {
            value: Arc::clone(&self.value),
        }
    }
}

fn main() {
    let counter = SharedCounter::new();
    let mut handles = Vec::new();

    for _ in 0..3 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter_clone.inc();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.get());
}
