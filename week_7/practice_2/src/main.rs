use std::io;

fn main() {
    println("Welcome! this checks if char is a digit or not");
    checker()
}

fn checker() {
    println!("Enter a character: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && <= '9' {
        println!("Character '{}' is a digit", ch);

    }
    else{
        println!("Character is not a digit", ch);
    }
}
