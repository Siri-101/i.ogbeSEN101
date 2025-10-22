use std::io;

fn main() {


    let mut input2 = String::new();
    println!("Enter Lowerbound: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let  lower_bound:i32 = input2.trim().parse().expect("Input not an integer");

    
    let mut input = String::new();
    println!("Enter Upperbound: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let  upper_bound:i32 = input.trim().parse().expect("Input not an integer");

    for x in lower_bound..upper_bound {
        println!("Count Level is: {}", x);
    }
}
