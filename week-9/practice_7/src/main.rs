//Rust structures
struct Employee {
    name: String,
    company: String,
    age: u32
}

fn main() {
    let empl = Employee {
        company: String::from("Enrst & Young"),
        name: String::from("Ebibiong Jessica"),
        age:25
    };
    println!("Name = {} \n", empl.name);
    println!("Company = {} \n", empl.company);
    println!("Age = {} \n", empl.age);
}
