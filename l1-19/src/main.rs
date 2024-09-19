use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a sentence: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Error reading input");

    let input = input.trim();

    let reversed = reverse_words(input);

    // Outputting the result
    println!("Reversed words: {}", reversed);
}

// Function to reverse the words in a sentence
fn reverse_words(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}
