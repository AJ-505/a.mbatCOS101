use std::io;
use std::io::Write;
use std::fs::File;

fn input(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input from terminal");
    input
}
fn push_inputs(vector: &mut Vec<String>, beer_type: i32) {
    let mut drink_numbers:usize;
    loop {
        drink_numbers = input("How many drink names would you like to store").trim().parse().expect("Please enter a positive whole number");
        match drink_numbers {
            drink_numbers if drink_numbers > 50 => {println!("The number is too large. Please do not enter a value more than 50")}
            _ => {break}
        }
    }
    let beer_types = vec!["Lager", "Stout", "Non-Alcoholic"];
    for _i in 0..drink_numbers {
        let drink_name = input("Enter drink name: ");
        vector.push(drink_name);
    }
    let mut file = File::create("data.txt").expect("Could not create file");
    file.write_all(format!("The brand of drink stored is {} and its drink names are:\n", beer_types[(beer_type as usize)-1].trim()).as_bytes()).expect("Could not write to file");
    for i in 0..drink_numbers{
        file.write_all(vector[i as usize].as_bytes()).unwrap()
    }
}
fn main() {
    println!("Welcome to Nigerian Breweries Plc!\n");
    let (mut lager, mut stout, mut nonalcoholic) = (vec![],vec![],vec![]);
    let mut beer_type: String;
    loop {
        beer_type = input("What brand of beer would you like to enter? Is it lager(press 1), stout(press 2), nonalcoholic(press 3)").trim().to_string();
        match beer_type.as_str() {
            "1" => {
                push_inputs(&mut lager, beer_type.parse::<i32>().unwrap());
                break;
            }
            "2" => {
                push_inputs(&mut stout, beer_type.parse::<i32>().unwrap());
                break;
            }
            "3" => {
                push_inputs(&mut nonalcoholic, beer_type.parse::<i32>().unwrap());
                break;
            }
            _ => {
                println!("You have not entered a valid input.");
            }
        }
    }
    println!("Thank you for using this program!");
}
