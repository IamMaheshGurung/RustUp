

use std::env;
use std::fs;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 {
        //read from the file
        match fs::read_to_string(&args[1]) {
            Ok(success) => success,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                return;
            }
        }
    } else {
        //read from std in
        println!("ENter the text (ctrl + D or ctrl + z to end):");

        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read from stdin");
        input
    };

    analyze_text(&input);

}


fn analyze_text(text: &str) {
    let line_count = count_lines(text);
    let word_count = count_words(text);
    let char_count = count_chars(text);
    let frequency = word_frequency(text);

    println!("Lines: {}", line_count);
    println!("Words: {}", word_count);
    println!("Characters: {}", char_count);
    println!("Word Frequency: {:?}", frequency);
}



fn count_lines(text: &str) -> usize {
    text.lines().count()
}


fn count_words(text: &str) -> usize {
    text.split_whitespace()
        .count()
}

fn count_chars(text: &str) -> usize {
    text.chars().count()
}

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut frequency = HashMap:: new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();
        if word.is_empty() {
            continue;
        }
        *frequency.entry(word).or_insert(0) += 1;
    }
    frequency
}

