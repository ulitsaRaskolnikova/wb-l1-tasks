use std::collections::HashSet;

fn are_characters_unique(input: &str) -> bool {
    let mut visited = HashSet::new();

    for character in input.to_lowercase().chars() {
        // Проверяем, был ли символ ранее
        if !visited.insert(character) {
            return false;
        }
    }

    true 
}

fn main() {
    let samples = vec!["abcd", "abCdefAaf", "aabcd", "aA"];

    for sample in samples {
        let is_unique = are_characters_unique(sample);
        println!("\"{}\" contains unique characters: {}", sample, is_unique);
    }
}
