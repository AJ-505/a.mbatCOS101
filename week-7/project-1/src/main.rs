use std::io;
use std::thread::sleep;
use std::time::Duration; // These are for the functionality of pausing the program for a specific number of seconds.

fn process(vector:Vec<&str>){
    let mut output:&str = "";
    let mut experience_years:u32 = (parsed(input("How many years of experience do you have?").as_str())) as u32;
    output = match experience_years {
        experience_years if experience_years < 3 => vector[0],
        experience_years if experience_years < 6 => vector[1],
        experience_years if experience_years < 9 => vector[2],
        experience_years if experience_years < 11 => vector[3],
        experience_years if experience_years < 14 => vector[4],
        _ => vector[5],
    };
    let first_char = output.chars().nth(0).unwrap().to_lowercase().next().unwrap();
    if first_char == 'a' || first_char == 'e' || first_char == 'i' || first_char == 'o' || first_char == 'u' {
        println!("You are an {}", output);
    } else {
        println!("You are a {}", output);
    }
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
fn delay_program(seconds: u64) {
    let duration = Duration::new(seconds,0);
    sleep(duration);
}

fn run() {
    println!("Welcome to the Public Service APS level checker!");
    loop {
        let mut staff_category_number = parsed(input("Are you an office admin(press 1), academic staff(press 2), lawyer(press 3), teacher(press 4)").as_str());
        let mut result: &str;
        match staff_category_number {
            1 => {
                process(vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"]);
                break;
            },
            2 => {
                process(vec!["None", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"]);
                break;
            }
            3 => {
                process(vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"]);
                break;
            }
            4 => {
                process(vec!["Placement", "Classroom teacher", "Senior teacher", "Leading teacher", "Deputy Principal", "Principal"]);
                break;
            }
            _ => { println!("Invalid input format"); }
        }
    };

}

fn main() {
    run();
    loop {
        println!("Do you want to re-run this program for another user? (Y/N)");
        let mut response = String::new();
        response.clear();
        io::stdin().read_line(&mut response).expect("Failed to read input");
        let response = response.trim().to_uppercase();

        if response == "Y" {
            println!("Thank you for your response! Restarting program....");
            delay_program(3);
            run();
        } else if response == "N" {
            println!("Thank you for using my program!");
            break;
        }  else {
            println!("Invalid input. Please enter Y or N.");
        }
    }
}