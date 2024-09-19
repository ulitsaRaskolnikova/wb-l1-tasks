use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use dashmap::DashMap;
use std::fmt::Display;

fn main() {
    let data = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut map = data.lock().unwrap();
            map.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Результат с использованием Mutex и HashMap: {:?}", data.lock().unwrap());


    let data = DashMap::with_capacity(10);

    let mut handles = vec![];

    for i in 0..10 {
        let data = data.clone();
        let handle = thread::spawn(move || {
            data.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Результат с использованием DashMap: {:?}", data);
}
