fn main() {
    let v = vec![10,20,30,40,50];
    let v2 = v; // moves ownership from v to v2
    display(v2.clone());
    //v2 is moved to display and v2 is invalidated
    println!("In main {:?}", v2);
    //v2 is no longer usable here
}

fn display(v:Vec<i32>) {
    println!("Inside display {:?}", v);
}