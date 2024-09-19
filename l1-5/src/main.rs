use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};
use std::io;

use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let mut workers = String::new();
    
    // Читаем количество воркеров из стандартного ввода
    io::stdin()
        .read_line(&mut workers)
        .expect("Failed to read line");
    
    let workers: usize = workers.trim().parse().unwrap();

    // Создаем пул потоков и оборачиваем его в Arc и Mutex для безопасного доступа из нескольких потоков
    let pool = Arc::new(Mutex::new(ThreadPool::new(workers)));

    // Создаем атомарную переменную для управления состоянием работы программы
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Устанавливаем обработчик для Ctrl-C, чтобы остановить программу
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut i = 0;
    // Бесконечный цикл для отправки сообщений в пул
    while running.load(Ordering::SeqCst) {
        let message = format!("Message {}", i); // Формируем сообщение
        pool.lock().unwrap().execute(message); // Отправляем сообщение в пул
        i += 1; // Увеличиваем счетчик
        thread::sleep(std::time::Duration::from_millis(500)); // Задержка для демонстрации работы
    }

    println!("Program exiting, cleaning up..."); // Сообщение о завершении программы
}

pub struct ThreadPool {
    workers: Vec<Worker>, // Вектор воркеров
    sender: Option<mpsc::Sender<Job>>, // Канал для отправки задач
}

type Job = Option<String>; // Тип задания

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // Проверяем, что размер пула больше 0

        let (sender, receiver) = mpsc::channel(); // Создаем канал для обмена сообщениями

        let receiver = Arc::new(Mutex::new(receiver)); // Оборачиваем приемник в Arc и Mutex

        let mut workers = Vec::with_capacity(size); // Создаем вектор для воркеров

        // Создаем воркеров
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); // Добавляем воркера в вектор
        }

        ThreadPool {
            workers,
            sender: Some(sender), // Возвращаем новый пул с отправителем
        }
    }

    // Метод для выполнения задачи
    pub fn execute(&self, message: String) {
        let job = Some(message); // Оборачиваем сообщение в тип Job
        self.sender.as_ref().unwrap().send(job).unwrap(); // Отправляем сообщение в канал
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Dropping ThreadPool, cleaning up workers..."); // Сообщение о завершении работы пула
        drop(self.sender.take()); // Закрываем канал, чтобы предотвратить дальнейшую отправку задач

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id); // Сообщение о завершении работы воркера
            // После вызова take(), Option становится равным None, что предотвращает дальнейшие попытки отправки задач через закрытый канал
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); // Ждем завершения потока
            }
        }
        println!("All workers have been shut down."); // Сообщение о завершении всех воркеров
    }
}

struct Worker {
    id: usize, // Идентификатор воркера
    thread: Option<thread::JoinHandle<()>>, // Объект для управления потоком
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv(); // Получаем сообщение из канала
            match message {
                Ok(message) => {
                    match message {
                        Some(message) => println!("Worker {} received: {}", id, message), // Сообщение о получении данных
                        None => println!("Worker {} received nothing", id), // Сообщение о том, что ничего не получено
                    }
                    thread::sleep(std::time::Duration::from_millis(500)); // Задержка для имитации работы
                }
                Err(_) => break, // Выходим из цикла при ошибке
            }
        });

        Worker {
            id,
            thread: Some(thread), // Возвращаем нового воркера с потоком
        }
    }
}
