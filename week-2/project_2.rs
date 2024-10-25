fn main() {
let (toshiba, mac, hp, dell, acer) = (450_000.00 * 2.0, 1_500_000.00, 750_000.00 * 3.0, 2_850_000.00 * 3.0, 250_000.00); // Single-line multi-variable listing in Rust
let sum = toshiba + mac + hp + dell + acer;
let average = sum / 10.0;

println!("The sum of P.M Okeke and Sons Ltd's sales is {}.", sum);
println!("The average of P.M Okeke and Sons Ltd's sales is {}.", average);
}
