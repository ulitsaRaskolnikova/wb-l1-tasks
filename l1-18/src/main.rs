use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Error reading input");
    
    let input = input.trim();

    let reversed = reverse_string(input);

    println!("Reversed string: {}", reversed);
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
