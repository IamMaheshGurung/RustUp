use std::env::args;
use minigrep::Config;
use std::process;





fn main() {
    let args: Vec<String> = args().collect();


    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    
    

    
    println!("Searching for {} in file {}", config.query, config.file_path);
    
    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
    


    
}

