fn main(){
    let principal: f64 = 520_000_000.00;
    let rate: f64 = 10.00;
    let time: f64 = 5.00;

    let amount: f64 = principal * ((1.0 + (rate / 100.0))).powf(time);
    let interest: f64 = amount - principal;

    println!("The compound interest for 5 years at 10% per annum compounded annually is {}.", interest);
}
