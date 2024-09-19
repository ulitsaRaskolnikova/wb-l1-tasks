use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a sentence: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Error reading input");

    let input = input.trim();

    let reversed = reverse_words(input);

    println!("Reversed words: {}", reversed);
}

fn reverse_words(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}
