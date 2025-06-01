
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcoome to median Number finder!");
    println!("Please enter a list of numbers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input
    .trim()
    .split_whitespace()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();

    println!("OKey median or mode ?");
    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice)
    .expect("Failed to get choice");

    match choice.as_str().trim() {
        "median" => {
            match find_median(&numbers) {
                Some(median) => println!("The median is: {}", median),
                None => println!("No numbers were provided."),
            }
        },
        "mode" => {
            let mode = find_mode(&numbers);
            println!("The mode is: {}", mode);
        },
        _ => {
            println!("Invalid choice. Please enter 'median' or 'mode'.");
        }
    }

}


fn find_median(numbers: &Vec<i32>) -> Option<f64> {
    let mut sorted = numbers.clone();

    sorted.sort();
    let len = sorted.len();

    if len == 0 {
        return None;
    }
    if len % 2 == 1 {
        Some(sorted[len /2] as f64)
    } else {
        Some((sorted[len / 2 - 1] + sorted[len / 2]) as f64 / 2.0)
    }

}

fn find_mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for &num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode = numbers[0];
    let mut max_count = 0;
    for (&num, &count) in &map {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }
    mode
}
