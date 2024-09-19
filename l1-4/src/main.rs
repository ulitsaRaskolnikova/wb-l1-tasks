use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
fn main() {
    let pool = ThreadPool::new(5);
    let mut i = 0;
    loop {
        let message = format!("Message {}", i);
        pool.execute(message);
        i += 1;
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<String>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn execute(&self, message: String) {
        self.sender.send(message).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<String>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(message) => {
                    println!("Consumer {} received: {}", id, message); 
                    thread::sleep(std::time::Duration::from_secs(1));
                }
                Err(_) => break,
            }
        });

        Worker { id, thread }
    }
}