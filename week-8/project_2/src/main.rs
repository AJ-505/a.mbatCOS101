use std::io;
use std::thread::sleep;
use std::time::Duration; // These are for the functionality of pausing the program for a specific number of seconds.
use std::fs::OpenOptions;
use std::io::Write;

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

fn main() {
    //Initialize variables
    let (mut student_names, mut matric_nos, mut student_depts, mut student_levels) = (vec![], vec![], vec![], vec![]);
    let mut student_numbers:i32;

    //Handle files
    let mut file = OpenOptions::new().append(true).open("student_data.csv").expect("Failed to open student_data.csv");
    println!("Welcome to the PAU student information management system!");
    loop {
        let student_numbers_str = input("How many student details would you like to enter?");
        match student_numbers_str.parse::<i32>() {
            Ok(s) => {
                if s < 1 {
                    println!("That is not a valid input.");
                } else {
                    student_numbers = s;
                    break;
                }
            }
            Err(_) => {println!("That is not a valid input.");}
        }
    }

    for i in 0..student_numbers {
        student_names.push(input("Hello Student! What's your name?"));
        matric_nos.push(input("What is your matriculation number?"));
        student_depts.push(input("What is your department?"));
        loop {
            let student_level_string = input("What is your level?");
            match student_level_string.parse::<i32>() {
                Ok(level) => {
                    if level % 100 == 0 && level > 0 && level < 501{
                        student_levels.push(level);
                        break; // Only break the loop and push if it's a valid number that's divisible by 100
                    } else {
                        println!("You have not entered a valid level.");
                    }
                }
                Err(_) => {println!("You have not entered a valid level.");}
            }
        }
        let temp = format!("{}, {}, {}, {}\n", student_names[i as usize], matric_nos[i as usize], student_depts[i as usize], student_levels[i as usize]);
        file.write_all(temp.as_bytes()).expect("Failed to write to file");
        if i != student_numbers - 1 {
            println!("Loading program for next student...");
            delay_program(2);
        }
    }
    println!("The results have been successfully saved to the file student_data.csv.");
    println!("Thank you for using my program!");
}