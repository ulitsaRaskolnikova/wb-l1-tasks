use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use std::io;

use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let mut workers = String::new();
    
    io::stdin()
        .read_line(&mut workers)
        .expect("Failed to read line");
    
    let workers: usize = workers.trim().parse().unwrap();

    let pool = Arc::new(Mutex::new(ThreadPool::new(workers)));

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut i = 0;
    while running.load(Ordering::SeqCst) {
        let message = format!("Message {}", i);
        pool.lock().unwrap().execute(message);
        i += 1;
        thread::sleep(std::time::Duration::from_millis(500)); // Задержка для демонстрации работы
    }

    println!("Program exiting, cleaning up...");
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Option<String>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute(&self, message: String) {
        let job = Some(message);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Dropping ThreadPool, cleaning up workers...");
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // После того, как мы вызвали take(), Option становится равным None, и это предотвращает дальнейшие попытки отправки задач через закрытый канал
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        println!("All workers have been shut down.");
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(message) => {
                    match message {
                        Some(message) => println!("Worker {} received: {}", id, message),
                        None => println!("Worker {} received nothing", id),
                    }
                    thread::sleep(std::time::Duration::from_millis(500));
                }
                Err(_) => break,
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
