use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("Welcome to the globacom structure management program.");
    println!("What type of user are you?\n
Note: Administrator   - A or 1,\n
      Project manager - P or 2,\n
      Employee        - E or 3,\n
      Customer        - C or 4, \n
      Vendor          - V or 5
");

let matches = vec!["a", "p", "e", "c", "v", "1", "2", "3", "4", "5"];
let mut response = String::new();
loop {
    io::stdin().read_line(&mut response).expect("Unable to get your user response");
    response = response.trim().to_lowercase();
    if matches.contains(&&response.as_str()) {
        break;
    }
    else {
        println!("You have not entered a valid input.");
    }
    response.clear();
}

let response = response.as_str();
if response == "a" || response == "1"{
    display("globacom_db.sql");
} 
else if response == "p" || response == "2"{
    display("project_tb.sql");
}
else if response == "e" || response == "3"{
    display("staff_tb.sql");
}
else if response == "c" || response == "4"{
    display("customer_tb.sql");
}
else{
    display("globacom_db.sql");
}

println!("Thank you so much for using this program.");
}

fn display(path: &str) {
    let mut file = File::open(path).expect("Unable to retrieve file contents.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
}