fn main() {
    let x = 5;



    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }


    let my_detail:(u8, f64, u32) = (1, 24.89, 3);

    let (age, salary, service_year) = my_detail;
   // println!("The value of a is: {age}");
    //   println!("The value of b is: {salary}");
    //println!("The value of c is: {service_year}");
    println!("The value we get here is : {}", my_detail.0);



}
