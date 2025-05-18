fn main() {
    println!("HI WELCOME TO TEMPRETURE CONVERTER");
    println!("Select 1 to convert Celsius to Fahrenheit\nSelect 2 to convert Fahrenheit to Celsius");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };



}
