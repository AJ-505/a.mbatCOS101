fn main() {
    let v = vec![101, 250, 330, 400];
    //vector v owns the object in the heap
    //only a single variable owns the heap memory at any given time
    let v2=v;
    println!("{:?}", v2); // v was changed to v2 to ensure it runs
}