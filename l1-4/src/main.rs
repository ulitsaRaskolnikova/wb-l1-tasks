use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    io,
};

fn main() {
    let mut workers = String::new();
    
    // Читаем количество воркеров из стандартного ввода
    io::stdin()
        .read_line(&mut workers)
        .expect("Не удалось прочитать строку");
    
    let workers: u32 = workers.trim().parse().unwrap();

    // Создаем пул потоков с указанным количеством воркеров
    let pool = ThreadPool::new(workers as usize);
    let mut i = 0;

    // Бесконечный цикл для отправки сообщений
    loop {
        let message = format!("Message {}", i); // Формируем сообщение
        pool.execute(message); // Отправляем сообщение в пул
        i += 1; // Увеличиваем счетчик
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>, // Вектор воркеров
    sender: mpsc::Sender<String>, // Канал для отправки сообщений
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // Убеждаемся, что размер пула больше 0

        let (sender, receiver) = mpsc::channel(); // Создаем канал

        let receiver = Arc::new(Mutex::new(receiver)); // Оборачиваем приемник в Arc и Mutex для безопасного доступа из потоков

        let mut workers = Vec::with_capacity(size); // Создаем вектор для воркеров

        // Создаем воркеры
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); // Добавляем воркера в вектор
        }

        ThreadPool { workers, sender } // Возвращаем новый пул
    }

    // Метод для выполнения задачи
    pub fn execute(&self, message: String) {
        self.sender.send(message).unwrap(); // Отправляем сообщение в канал
    }
}

struct Worker {
    id: usize, // Идентификатор воркера
    thread: thread::JoinHandle<()>, // Объект для управления потоком
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<String>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv(); // Получаем сообщение из канала
            match message {
                Ok(message) => {
                    println!("Consumer {} received: {}", id, message); 
                    thread::sleep(std::time::Duration::from_secs(1)); // Задержка для имитации работы
                }
                Err(_) => break, // Если произошла ошибка, выходим из цикла
            }
        });

        Worker { id, thread } // Возвращаем нового воркера
    }
}
