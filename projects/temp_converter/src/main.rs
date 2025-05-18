use std::io;



fn main() {
    println!("HI WELCOME TO TEMPRETURE CONVERTER");
    println!("Select 1 to convert Celsius to Fahrenheit\nSelect 2 to convert Fahrenheit to Celsius\n Select other to cancel");

    let mut uinput = String::new();

    io::stdin()
        .read_line(&mut uinput)
        .expect("Failed to read line");

    let uinput : u32 = match uinput.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };


    if uinput == 1 {
        println!("Enter the temperature in Celsius");
        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read the celsius temperature");

        let celsius: i32 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid number as temperature");
                return;
            }

        };
        let fahrenheit = (celsius * 9/5) + 32;
        println!("The temperature in Fahrenheit is {}", fahrenheit);

    } else if uinput == 2 {
        println!("ENter the temperature in Fahrenheit");
        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read the fahrenheit temperature");

        let fahrenheit: i32 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid number as temperature");
                return;
            }
        };
        let celsius = (fahrenheit - 32) * 5/9;
        println!("The temperature in Celsius is {}", celsius);
    } else {
        println!("You have cancelled the program");
    }
    println!("Thank you for using the temperature converter");
    println!("Have a nice day");
}




