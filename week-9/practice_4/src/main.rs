fn main() {
    //Returning a value from a function
    let v = vec![20,40,60,80];

    let v2=v.clone(); // v.clone() is used instead of v
    let v2_return = display(v2);
    println!("In main {:?}", v);
}

fn display(v:Vec<i32>) -> Vec<i32> {
    //returning the same vector
    println!("Inside display {:?}", v);
    return v;
}