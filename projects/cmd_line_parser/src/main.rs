use std::io::{self, Write};

//Enum for the command 
enum Command {
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Show(String),
    Quit,
    Unknown,

}

fn parse_command(input:&str) -> Command {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();


    match parts.as_slice() {
        ["add", x, y] => {
            let a = x.parse::<i32>().ok();
            let b = y.parse::<i32>().ok();
            match(a, b) {
                (Some(a), Some(b)) => Command::Add(a, b),
                _ => Command::Unknown,
            }
        }
        ["sub", x, y] => {
            let a = x.parse::<i32>().ok();
            let b = y.parse::<i32>().ok();
            match(a, b) {
                (Some(a), Some(b)) => Command::Sub(a, b),
                _ => Command::Unknown,
            }
        }
        ["mul", x, y] => {
            let a = x.parse::<i32>().ok();
            let b = y.parse::<i32>().ok();
            match(a, b) {
                (Some(a), Some(b)) => Command::Mul(a, b),
                _ => Command::Unknown,
            }
        }
        ["show", name] => Command::Show(name.to_string()),
        ["quit"] => Command::Quit,
        _ => Command::Unknown,

    }
}

fn execute_command(command: Command) {
    match command {
        Command::Add(a, b) => println!("Result: {}", a + b),
        Command::Sub(a, b) => println!("Result: {}", a - b),
        Command::Mul(a, b) => println!("Result: {}", a * b),
        Command::Show(name) => println!("Showing: {}", name),
        Command::Quit => {
            println!("Quitting the program.");
            std::process::exit(0);
        }
        Command::Unknown => println!("Unknown command. Please try again."),
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("failed to read input");
            continue;
        }

        let command = parse_command(&input);
        if let Command::Quit = command {
            execute_command(command);
            break; // Exit the loop if the command is Quit
        }
        execute_command(command);
    }
}
