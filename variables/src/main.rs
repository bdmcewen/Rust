fn main() {

    // declaring a mutable variable "x"
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //example of shadowing
    x = 5;

    let x = x + 1;

    let x = x * 2;

    println!();

    println!("The value of x is: {}", x);

    
    const MAX_POINTS: u32 = 100_000;

    println!("{}", MAX_POINTS);


}
