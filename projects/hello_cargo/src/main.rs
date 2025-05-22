fn main() {
    let s1 = String::from("hello");
    takes_ownership(s1.clone()); // s1 is moved into the function
    println!("s1: {}", s1); // This line would cause a compile-time error because s1 has been moved
    // println!("s1: {}", s1); // This line would cause a compile-time error because s1 has been moved

    let x = 5;
    makes_copy(x);
    println!("x: {}", x); // This line works because i32 is a Copy type   
    

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("s1: {}", s1); // s1 is valid here
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved into the function
   // println!("s2: {}", s2); // This line would cause a compile-time error because s2 has been
    

    println!("s3: {}", s3); // s3 is valid here
}


fn takes_ownership(s: String) {
    println!("s: {}",s);
}

fn makes_copy(i : i32) {
    println!("i: {}", i);
}


fn gives_ownership() -> String {
    let s = String::from("hello");
    s // s is returned and moved out of the function
}


fn takes_and_gives_back(s: String) -> String {
    println!("s: {}", s);
    s // s is returned and moved out of the function
}
