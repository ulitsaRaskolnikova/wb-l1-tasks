use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut unique_lines = Vec::new();

    for input in stdin.lock().lines() {
        if let Ok(line) = input {
            if unique_lines.iter().all(|existing| existing != &line) {
                unique_lines.push(line); 
            }
        }
    }

    // cltrl d чтобы остановить ввод

    for line in &unique_lines {
        println!("{}", line);
    }
}
