use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Введите строку: ");
    io::stdout().flush().expect("Не удалось очистить stdout");
    io::stdin().read_line(&mut input).expect("Ошибка при чтении строки");

    let input = input.trim();
    let reversed = reverse_string(input);
    println!("Перевернутая строка: {}", reversed);
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
