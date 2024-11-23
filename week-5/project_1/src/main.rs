use std::io;

fn main() {
    let poundo:f32 = 3200.00;
    let fried:f32 = 3000.00;
    let amala:f32 = 2500.00;
    let eba:f32 = 2000.00;
    let wrice:f32 = 2500.00;
    println!(
        "-------------------------------------------------
        Menu                          | Price
--------------------------------------+-----------
P = Poundo Yam / Edikang-ikong soup   |- N3,200
F = Fried Rice & Chicken              |- N3,000
A = Amala & Ewedu Soup                |- N2,500
E = Eba & egusi Soup                  |- N2,000
W = White Rice & Stew                 |- N2,500
-------------------------------------------------"
    );
    let mut amount:f32 = 0.0;
    let mut food = String::new();
    let mut quantity = String::new();

    println!("Input food (Note: Valid input are P, F, A, E & W)");
    io::stdin().read_line(&mut food).expect("Invalid input");
    let food = food.trim();

    println!("Input quantity: ");
    io::stdin().read_line(&mut quantity).expect("Invalid input");
    let quantity:f32 = quantity.trim().parse().expect("Input is not a number");

    if food == "P" {
        amount = poundo * quantity;
    } else if food == "F" {
        amount = fried * quantity;
    } else if food == "A" {
        amount = amala * quantity;
    } else if food == "E" {
        amount = eba * quantity;
    } else if food == "W" {
        amount = wrice * quantity;
    }

    if amount >= 10000.00 {
        amount = 0.95 * amount;
    }

    println!("Your amount to pay is {}", amount);

}