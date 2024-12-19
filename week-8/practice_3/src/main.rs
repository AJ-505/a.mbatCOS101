use std::fs;
fn main() {
    fs::remove_file("data.txt").expect("could not remove file");
    println!("File is removed");
}
