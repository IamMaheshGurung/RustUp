
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n -1) + fibonacci(n - 2)
    }
}


fn main() {
    let n = 10;
    for i in 0..n {
        println!("Fibonacci of {} is {}", i, fibonacci(i));
    }
}

