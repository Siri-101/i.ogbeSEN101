use std::io;

fn main() {
    let mut is_experienced = String::new();

    println!("Is the Employee Experienced (Y/N): ");
    io::stdin().read_line(&mut is_experienced).expect("Could not Read Input");
    if is_experienced.trim() == "Y" {
        let mut input2 = String::new();
        println!("Age: ");
        io::stdin().read_line(&mut input2).expect("Could not Read Input");
        let age:u16 = input2.trim().parse().expect("Is not an Integer");

        if age >= 40 {
            println!("Incentive: N1,560,000");
        }
        else if age >= 30 && age < 40 {
            println!("Incentive: N1,480,000");
        }
        else {
            println!("Incentive: N1,300,000");
        }

    }
    else if is_experienced.trim() == "N"{
        println!("Incentive: N100,000");
    }
    else {
        println!("Invalid Input. Type Y or N");
    }

    // let mut age = String::new();

    // println!("Age: ")
}
