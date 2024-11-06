//Rust program to get the roots of a quadratic equation
use std::io;

fn main() {
    // Get input for roots of equation
    println!("Please enter the value of a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Please enter the value of b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Please enter the value of c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = (b*b) - (4.0 * a * c);

    let root1: f32 = ((-1.0*b) + d.powf(0.5))/(2.0 * a);
    let root2: f32 = ((-1.0*b) - d.powf(0.5))/(2.0 * a);

    //Mpre optimization implementations
    if root1.to_string() == "NaN" || root2.to_string() == "NaN" {
        println!("There are no real roots for this equation");
    }
    else if root1 == root2 {
        println!("The root of this equation is {} ({} twice)", root1, root2);
    }
    else {
        println!("The roots of the equation are {} and {}", root1, root2);
    }
}
