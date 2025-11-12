use std::io;

fn main() {
    let mut input1 = String::new();
    println!("Enter Parameter A ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let  a:i32 = input1.trim().parse().expect("Input not an integer");

    
    let mut input2 = b::new();
    println!("Enter Upperbound: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let  b:i32 = input2.trim().parse().expect("Input not an integer");

    add(a, b)
}


fn add(a:i32, b:i32) {
    let sum = a + b;
    println!("Sum of a and b is: {}", sum)
}
