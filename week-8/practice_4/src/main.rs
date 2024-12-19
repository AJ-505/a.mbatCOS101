use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("Can't open file");
    file.write_all("\nHello Class".as_bytes()).expect("Can't write file");
    file.write_all("\nThis is the appendage to the document".as_bytes()).expect("Can't write file");
    println!("file append success!");
}
