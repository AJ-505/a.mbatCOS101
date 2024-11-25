use std::io;

fn trapezium() {
    let (mut height_str, mut base1_str, mut base2_str) = (String::new(), String::new(), String::new());

    println!("Input height of trapezium(in metres)");
    io::stdin().read_line(&mut height_str).expect("Error reading input");
    let height:u32 = height_str.trim().parse().expect("Not a valid number or input value");

    println!("Input 1st base of trapezium(in metres)");
    io::stdin().read_line(&mut base1_str).expect("Error reading input");
    let base1:u32 = base1_str.trim().parse().expect("Not a valid number or input value");

    println!("Input 2nd base of trapezium(in metres)");
    io::stdin().read_line(&mut base2_str).expect("Error reading input");
    let base2:u32 = base2_str.trim().parse().expect("Not a valid number or input value");

    let area:f32 = ((height as f32) / 2.0) * (base1 + base2) as f32;
    println!("The area of the trapezium is {} square metres", area);
}
fn rhombus() {
    let (mut diagonal1_str, mut diagonal2_str) = (String::new(), String::new());

    println!("Input 1st diagonal of rhombus(in metres)");
    io::stdin().read_line(&mut diagonal1_str).expect("Error reading input");
    let diagonal1:u32 = diagonal1_str.trim().parse().expect("Not a valid number or input value");

    println!("Input 2nd diagonal of trapezium(in metres)");
    io::stdin().read_line(&mut diagonal2_str).expect("Error reading input");
    let diagonal2:u32 = diagonal2_str.trim().parse().expect("Not a valid number or input value");

    let area:f32 = (diagonal1 * diagonal2) as f32 * 0.5;
    println!("The area of the rhombus is {} square metres", area);
}

fn parallelogram() {
    let (mut base_str, mut altitude_str) = (String::new(), String::new());

    println!("Input base of parallelogram(in metres)");
    io::stdin().read_line(&mut base_str).expect("Error reading input");
    let base:u32 = base_str.trim().parse().expect("Not a valid number or input value");

    println!("Input altitude of parallelogram");
    io::stdin().read_line(&mut altitude_str).expect("Error reading input");
    let altitude:u32 = altitude_str.trim().parse().expect("Not a valid number or input value");

    let area:f32 = (base * altitude) as f32;
    println!("The area of the parallelogram is: {} square metres", area);
}

fn cube() {
    println!("Enter the length of the side of the cube (in metres)");
    let mut side_str = String::new();
    io::stdin().read_line(&mut side_str).expect("Error reading input");
    let side:u32 = side_str.trim().parse().expect("Not a valid number or input value");
    let volume:f32 = 6.0 * (side * side) as f32;
    println!("The volume of your cube is {} square metres", volume);
}
fn cylinder() {
    let (mut height_str, mut radius_str) = (String::new(), String::new());

    println!("Input radius of cylinder(in metres)");
    io::stdin().read_line(&mut radius_str).expect("Error reading input");
    let radius:u32 = radius_str.trim().parse().expect("Not a valid number or input value");

    println!("Input height of cylinder(in metres)");
    io::stdin().read_line(&mut height_str).expect("Error reading input");
    let height:u32 = height_str.trim().parse().expect("Not a valid number or input value");

    let volume:f32 = 22.0/7.0 * (radius * radius) as f32 * height as f32;
    println!("The volume of the cylinder is {} square metres", volume);
}
fn main() {
    println!("Hello, user!");
    let mut shape = String::new();
    let mut condition:bool = false;
    while !condition {
        println!("Welcome to my calculator program! What calculation(s) will you like to perform? Note:
T = Trapezium, R = Rhombus, P = Parallelogram, C = Cube, B = Cylinder");
        io::stdin().read_line(&mut shape).expect("Failed to read line");
        shape = shape.trim().to_uppercase();
        condition =  shape == "T" || shape == "R" || shape == "P" || shape == "C" || shape == "B";
        if !condition {
            println!("You have not entered a valid input. Please enter a valid input.");
        } else {
            if shape == "T" {
                trapezium();
            } else if shape == "R" {
                rhombus();
            } else if shape == "P" {
                parallelogram();
            } else if shape == "C" {
                cube();
            } else if shape == "B" {
                cylinder();
            }
        }
    }
}
