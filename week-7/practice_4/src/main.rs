fn main() {
    //Name vector
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    //Age vector
    let age = vec![16, 17, 19, 22, 20, 21, 10, 23];
    print!("Age allocation: \n");

    //Group to iterate elements in vector
    for i in 0..age.len() {
        print!("{} is {} years old\n ", name[i], age[i]);
    }

}
