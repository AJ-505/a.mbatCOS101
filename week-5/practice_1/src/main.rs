fn main() {
    let name = "Aisha Lawal";
    let uni = "Pan-Atlantic University";
    let addr = "Km 52 Lekki Epe Expresswy, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni, addr);

    let department: &'static str = "Computer Science";
    let school : &'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}", department, school);
}
