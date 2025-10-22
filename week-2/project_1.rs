fn main() {
    let principal: f64 = 520_000_000.0;
    let rate: f64 = 0.1;
    let time: f64 = 5.0;

    // (1 + rate).powf(time) is used for exponentiation with floating-point numbers
    let amount = principal * (1.0 + rate).powf(time);
    let compound_interest = amount - principal;

    println!("Compound Interest is ₦{:.2}", compound_interest);
}

