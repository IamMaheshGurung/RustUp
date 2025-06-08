use std::io::{self, Write};

fn main() {
    println!("Pig Latin Converter");
    println!("Enter a phrase to convert to Pig Latin:");
    
    // Get user input
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Convert to Pig Latin
    let pig_latin = convert_to_pig_latin(&input);
    
    println!("Pig Latin: {}", pig_latin);
}

fn convert_to_pig_latin(text: &str) -> String {
    // Split the input into words and convert each one
    text.trim()
        .split_whitespace()
        .map(convert_word_to_pig_latin)
        .collect::<Vec<String>>()
        .join(" ")
}

fn convert_word_to_pig_latin(word: &str) -> String {
    // Check if the word is empty
    if word.is_empty() {
        return String::new();
    }
    
    // Handle punctuation by preserving it at the end
    let mut punctuation = String::new();
    let mut word_chars: Vec<char> = word.chars().collect();
    
    while !word_chars.is_empty() && !word_chars.last().unwrap().is_alphabetic() {
        punctuation.insert(0, word_chars.pop().unwrap());
    }
    
    if word_chars.is_empty() {
        return punctuation;
    }
    
    // Get the first character to check if it's a vowel
    let first_char = word_chars[0].to_lowercase().next().unwrap();
    
    let result = if is_vowel(first_char) {
        // Word starts with a vowel - add "hay"
        format!("{}-hay", word_chars.iter().collect::<String>())
    } else {
        // Word starts with a consonant - move first letter to end and add "ay"
        let first = word_chars.remove(0);
        format!("{}-{}ay", word_chars.iter().collect::<String>(), first)
    };
    
    // Reattach punctuation
    result + &punctuation
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}