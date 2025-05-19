use std::io;


fn main() {
    println!("_________________________TODO LIST__________________________");
    println!("1. Add a task");
    println!("2. Remove a task");


    let  mut task =  vec![];

    


    loop {
        let mut input = String::new();

        println!("Please enter your choice (1-2): ");


        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        if guess == 1 {
            println!("Please enter the task you want to add: ");
            let mut task_input = String::new();
            io::stdin()
                .read_line(&mut task_input)
                .expect("Failed to read line");

            let task_input = task_input.trim().to_string();
            task.push(task_input);

            println!("Task added successfully!");



            println!("Do you want continue? if yes'y'");

            let mut wish = String::new();
            io::stdin()
                .read_line(&mut wish)
                .expect("Failed to read line");

            if wish.trim() != "y" {
                break
            }



        } else if guess == 2 {
            println!("Please enter the task you want to remove: ");
            let mut task_input = String::new();
            io::stdin()
                .read_line(&mut task_input)
                .expect("Failed to read line");

            let task_input = task_input.trim().to_string();

            if let Some(pos) = task.iter().position(|x| *x == task_input) {
                task.remove(pos);
                println!("Task removed successfully!");
            } else {
                println!("Task not found!");
            }

            println!("Do you want continue? if yes'y'");

            let mut wish = String::new();
            io::stdin()
                .read_line(&mut wish)
                .expect("Failed to read line");

            if wish.trim() != "y" {
                break
            }

        } else {
            println!("Please enter a valid choice");
        }
    }



}
