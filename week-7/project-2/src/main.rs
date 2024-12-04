use std::io;
use std::thread::sleep;
use std::time::Duration; // These are for the functionality of pausing the program for a specific number of seconds.
fn delay_program(seconds: u64) {
    let duration = Duration::new(seconds, 0);
    sleep(duration);
}

fn input(msg: &str) -> String {
    let mut input1 = String::new();
    println!("{}", msg);
    io::stdin().read_line(&mut input1).expect("Failed to read input from terminal");
    input1.trim().to_string()
}
fn parsed(input: &str) -> i32 {
    let parsed_value:i32 = input.trim().parse().expect("Failed to parse input properly");
    parsed_value
}
// This is to tell the compiler to ignore warnings for th
fn run_app() {
    println!("Welcome to my program!");
    let mut candidate_name = String::new();
    let mut top_candidate_experience:i32 = 0;
    let mut developer_names:Vec<String> = Vec::new();
    let mut experience_years = Vec::new();
    let mut developer_name:String;
    let mut no_of_developers:i32;
    loop {
        let no_of_developers_str = input("How many developers will you be interviewing today?");
        no_of_developers = parsed(no_of_developers_str.as_str());
        if no_of_developers < 1 {
            println!("You have not entered a valid input.");
        } else {
            break;
        }
    }

    for i in 1..=no_of_developers {
        loop {
            developer_name = input(format!("Hello developer {i}! What's your name?").as_str()).to_string();
            if developer_name.trim().len() <= 1 {
                println!("You have not entered a valid name.")
            } else if developer_name.trim().chars().all(|c| c.is_ascii_digit()) {
                println!("You cannot enter numeric values as input.")
            } else {
                developer_names.push(developer_name.clone());
                break; //
            }
        }
        loop {
            let mut developer_experience_years = parsed(input("How many years of programming experience do you have?").as_str());
            if developer_experience_years < 0 {
                println!("You cannot input negative values.");
            } else if developer_experience_years > 49 {
                println!("You cannot be a developer with more than 49 years of experience."); // the reason is that the maximum number of working years is 65 - 16 = 49(16 instead of 18 for universality)
            } else {
                experience_years.push(developer_experience_years);
                println!("Thank you for participating!\n");
                delay_program(1);
                break;
            }
        }
        if i != no_of_developers {
            println!("Loading program for next user..."); //Skip for last user as there is no next user
        }
        delay_program(2);
    }
    let mut counter: usize = 0;
    for i in experience_years {
        if i > top_candidate_experience {
            candidate_name = developer_names[counter].clone();
            top_candidate_experience = i;
        }
        counter += 1;
    }
    println!("The most experienced candidate is {} with {} years of experience.", candidate_name, top_candidate_experience);
}

fn main() {
    run_app();
    loop {
        println!("Do you want to re-run this program? (Y/N)");
        let mut choice = String::new();
        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        choice = choice.trim().to_lowercase();
        if  choice == "y" {
            println!("Thank you for your response! Restarting program...");
            delay_program(2);
            run_app();
        } else if choice == "n" {
            println!("Thank you for using my program!");
            break;
        } else {
            println!("You have not entered a valid input. Please input Y or N.");
        }
    }
}