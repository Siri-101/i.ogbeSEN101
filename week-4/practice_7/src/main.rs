use std::io;

fn main() {


    let mut input = String::new();
    println!("Enter a Number: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut  num:i32 = input.trim().parse().expect("Input not an integer");

    
    while num < 10 {
        println!("inside loop number value is {}", num);
        num += 1;
    }

    println!("Outside Loop Num value is {}", num)
}
