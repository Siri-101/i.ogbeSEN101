use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter Height (In Centimetres): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let  height:f32 = input.trim().parse().expect("Input not an integer");

    if height >= 150.0 && height <= 195.0 {
        println!("Average Height");
    }
    else if height > 170.00 && height <= 195.0 {
        println!("You are tall!");
    }
    else if height > 195.0 {
        println!("You are abnormally Tall!");
    }
    else {
        println!("You are a dwarf");
    }
}
