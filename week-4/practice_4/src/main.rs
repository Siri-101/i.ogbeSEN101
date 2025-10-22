use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("\n Please Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    println!("Your Name is {}", input1);

    println!("\n Please Enter your Age: ");
        io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");

    let age:i32 = input2.trim().parse().expect("Input not an integer");
    if age >= 18 {
        println!("Welcome to the party {}", input2)
    } else {
        println!("You are underage");
    }
}
