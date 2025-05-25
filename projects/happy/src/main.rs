fn main() {
    let pattern = std::env::args().nth(1).expect("Please provide a pattern to match");
    let text = std::env::args().nth(2).expect("Please provide a text to search in");

    println!("Pattern: {:?}, path: {:?}", pattern, text);
}
