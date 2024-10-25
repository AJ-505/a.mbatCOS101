fn main(){
    let principal:f64 = 210_000.00;
    let rate:f64 = 5.0;
    let time:f64 = 3.0;

    let depreciation:f64 = principal * (1.0 - (rate/100.0)).powf(time);
    let interest:f64 = depreciation - principal;
    let value:f64 = principal - depreciation; 

    println!("The amount of depreciation of the TV set is {}", depreciation);
    println!("The interest is {}", interest);
    println!("The value of the TV set after 3 years is {}", value);
}

