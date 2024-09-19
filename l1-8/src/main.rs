use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use dashmap::DashMap;

fn main() {
    let hashmap = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let hashmap = Arc::clone(&hashmap);
        let handle = thread::spawn(move || {
            let mut map = hashmap.lock().unwrap();
            map.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Результат с использованием Mutex и HashMap: {:?}", hashmap.lock().unwrap());


    let dashmap = Arc::new(DashMap::new());

    let mut handles = vec![];

    for i in 0..10 {
        let dashmap = Arc::clone(&dashmap);
        let handle = thread::spawn(move || {
            dashmap.insert(i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Вывод содержимого DashMap
    println!("Результат с использованием DashMap: {:?}", dashmap);
}
