use std::collections::HashMap;
use std::io;



fn main() {
   let mut rooms: HashMap<String, String> = HashMap::new();

    rooms.insert("Living Room".to_string(), "A cozy room with a fireplace.".to_string());
    rooms.insert("Kitchen".to_string(), "A room with a fridge and stove.".to_string());

    rooms.insert("Bedroom".to_string(), "A room with a bed and wardrobe.".to_string());

    println!(" The rooms in the house are:{}", rooms.len());

    println!("WHich room description do you want? ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); // Remove any trailing newline characters

    match input {
        "Living Room" => {
            println!("Description: {}", rooms.get("Living Room").unwrap());
        }
        "Kitchen" => {
            println!("Description: {}", rooms.get("Kitchen").unwrap());
        }
        "Bedroom" => {
            println!("Description: {}", rooms.get("Bedroom").unwrap());
        }
        _ => {

            println!("Room not found.");
            println!("Do you want it to register??");
            let mut new_room = String::new();
            io::stdin().read_line(&mut new_room).expect("Failed to read line");
            let new_room = new_room.trim(); // Remove any trailing newline characters
            if !new_room.is_empty() {
                println!("Enter the description for the new room:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim(); // Remove any trailing newline characters

                rooms.insert(new_room.to_string(), description.to_string());
                println!("New room '{}' with description '{}' added.", new_room, description);
            } else {
                println!("No room name provided. Exiting.");
            }
            return;
        }
    }
    
    

}
