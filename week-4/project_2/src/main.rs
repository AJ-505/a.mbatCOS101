use std::io;

fn main(){
    println!("Enter your experience: ");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Not a valid input");
    let experience:u32 = experience_input.trim().parse().expect("Not a valid number");

    println!("Enter your age: ");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Not a valid input");
    let age:i32 = age_input.trim().parse().expect("Not a valid number");

    let incentive = if experience >= 5 && age >= 40 {
        1_560_000
    } else if experience >= 5 && age >= 30 && age < 40 {
        1_480_000
    } else if experience >= 5 && age < 28 {
        1_300_000
    } else {
        0
    };

    println!("Your annual incentive is {}", incentive);
}