fn main() {
    //Using Vec::new()
    let v:Vec<i64> = Vec::new();

    //printing the size of the vector
    println!("\nThe length of the Vec::new is {}", v.len());

    //Using macros
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    //printing the size of the vector
    println!("\nThe length of the vector macro is {}", v.len());
}
