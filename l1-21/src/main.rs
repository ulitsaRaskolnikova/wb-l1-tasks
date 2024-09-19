use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    // Пример больших чисел
    let a_str = "1048577";
    let b_str = "2097154";

    // Парсинг чисел из строки
    let a = BigInt::from_str(a_str).expect("Failed to parse a");
    let b = BigInt::from_str(b_str).expect("Failed to parse b");

    // Выполнение операций
    let sum = &a + &b;
    let difference = &a - &b;
    let product = &a * &b;
    let quotient = &a / &b;
    let remainder = &a % &b;

    // Вывод результатов
    println!("a: {}", a);
    println!("b: {}", b);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}
