
use std::env;



enum Operation {
    Add, 
    Sub,
    Mul, 
    Div,
}


fn calculate(op: Operation, a: f64, b: f64) -> f64 {
    match op {
        Operation::Add => a + b,
        Operation::Sub => a - b,
        Operation::Mul => a * b,
        Operation::Div => {
            if b == 0.0 {
                panic!("Division by zero is not allowed.");
            }
            a / b
        }   
    }
}

fn parse_op(op_sign: &str) -> Option<Operation> {
    match op_sign {
        "+" => Some(Operation::Add),
        "-" => Some(Operation::Sub),
        "*" => Some(Operation::Mul),
        "/" => Some(Operation::Div),
        _ => None,
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: cargo run <num1> <op> <num2>");
        println!("Example: cargo run 10 + 5");

        return;

    }

    let a:f64 = args[1].parse::<f64>().expect("Invalid number");
    let op_sign:&String = &args[2];

    let b:f64 = args[3].parse::<f64>().expect("Invalid number");

    match parse_op(op_sign) {
        Some(op) => {
            let result = calculate(op, a, b);
            println!("Result: {}", result);

        }
        None => println!("Invalid operation symbole")
    }


}
