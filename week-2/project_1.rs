fn main() {
    let principal = 520_000_000;
    let rate = 0.1;
    let time = 5;


    let amount = principal * (1 + rate) * time

    let compound_interest = amount - principal;

    println!("Compound Interesr is {}", compound_interest);
}