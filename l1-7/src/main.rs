use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};
use tokio;
use tokio::task;
use tokio_util;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    println!("Способ 1: Остановка потоков через закрытие канала");

    let (tx, rx) = mpsc::channel();
    let worker_thread = thread::spawn(move || loop {
        match rx.recv() {
            Ok(message) => {
                println!("Thread received: {}", message);
            }
            Err(_) => {
                println!("Channel closed, stopping thread.");
                break;
            }
        }
    });

    // Посылаем несколько сообщений в поток
    tx.send("Message 1".to_string()).unwrap();
    tx.send("Message 2".to_string()).unwrap();
    
    // Останавливаем поток закрытием канала
    drop(tx);
    worker_thread.join().unwrap();

    println!("\nСпособ 2: Остановка задач Tokio с использованием CancellationToken");

    // Способ 2: Остановка задач Tokio с использованием CancellationToken
    let token = CancellationToken::new();
    let token_clone = token.clone();

    let tokio_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token_clone.cancelled() => {
                    println!("CancellationToken received, stopping Tokio task.");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    println!("Tokio task running...");
                }
            }
        }
    });

    // Даем задаче немного поработать
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Останавливаем задачу
    token.cancel();
    tokio_task.await.unwrap();

    println!("Все задачи завершены.");
}
