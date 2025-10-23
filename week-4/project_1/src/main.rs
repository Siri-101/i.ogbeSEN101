use std::io;

fn main() {
    println!("Program to help find roots:\n");

    println!("a: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Could not read Input");
    let a:f32 = input.trim().parse().expect("Not an Integer");

    println!("b: ");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not an Integer");
    let b:f32 = input2.trim().parse().expect("Not an Integer");

    println!("c: ");

    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Could not read Input");
    let c:f32 = input3.trim().parse().expect("Not an Integer");

    let discriminant:f32 = (b * b) as f32 - (4.0 * a * c) as f32;

    println!("Discriminant: {}", discriminant);
    if discriminant >= 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("The roots are: {} and {}", root1, root2);
    }
    else {
        println!("There are no real Roots");
        let root1_i = -b / (2.0 * a);
        let root1_ii = (-discriminant).sqrt()/((2.0 * a));

        let root2_i = -b / (2.0 * a);
        let root2_ii = (-discriminant).sqrt()/((2.0 * a));

        println!("The Complex roots are: {} + {}i  \n  and {} - {}i", root1_i, root1_ii, root2_i, root2_ii);

    }

    




}
