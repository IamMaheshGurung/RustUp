use std::io;
use rand::Rng;
use std::cmp::Ordering;




fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }

            };

        println!("Your guess is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
        
            }
        }
    }
}


    
