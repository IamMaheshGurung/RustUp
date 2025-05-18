use std::io;



fn main() {

    println!("___________________TEMPRETURE CONVERTER_______________");
    println!("********************WELCOME***************************");


    loop {
        println!("Select 1 to convert Celsious to fahrenheit");
        println!("Select 2 to convert Fahrenheit to celsious");
        println!("Select other to cancel");


        let mut uinput = String::new();


        io::stdin()
            .read_line(&mut uinput)
            .expect("Failed to read line");

        let uinput: i32 = match uinput.trim().parse() {
            Ok(num) => num, 
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }

        };

        if uinput == 1 {
            println!("ENter the number in Celsious please:  ");

            let mut celsious = String::new();


            io::stdin()
                .read_line(&mut celsious)
                .expect("failed to read the celsious");



            let celsious: i32 = match celsious.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;

                }
            };

            let fahrenheit = (celsious * 9 / 5) + 32;
            println!("The temperature in fahrenheit is: {}", fahrenheit);



        } else if uinput == 2 {
            println!("Enter the tempreture in fahrenheit:  ");

                let mut fahrenheit = String::new();

                io::stdin()
                .read_line(&mut fahrenheit)
                .expect("failed to read the fahrenheit");


                let fahrenheit: i32 = match fahrenheit.trim().parse() {
                    Ok(num)=> num, 
                    Err(_) => {
                        println!("Please enter a valid number.");
                        continue;
                    }


                };

                let celsious = (fahrenheit - 32) * 5 / 9;
                println!("The temperature in celsious is: {}", celsious);

        } else {
            println!("You have cancelled the program");
            println!("Do you still want to continue the progream");
            println!("If YES 'y' or 'Y' else 'n' or 'N' to exit the program");

            let mut continue_program = String::new();
            io::stdin()
                .read_line(&mut continue_program)
                .expect("failed to read the input");
            let continue_program = continue_program.trim();
            if continue_program == "y" || continue_program == "Y" {
                println!("You have selected to continue the program");
                continue;
            } else if continue_program == "n" || continue_program == "N" {
                println!("You have selected to exit the program");
                break;
            } else {
                println!("Invalid input, exiting the program.");
                break;
            }
            
        }
    }
    println!("Thank you for using the program");
}
