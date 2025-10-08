fn main(){
    let toshiba = 450_000;
    let dell = 2_850_000;
    let mac = 1_500_000;
    let hp = 750_000;
    let acer = 250_000;

    let sum = (2 * toshiba) + mac + (3 * hp) + (3 * dell) + acer;
    let total = 10;

    let average:f64 = sum as f64/total as f64;

    println!("The average price of a computer at Chief Donatus and Sons Ltd is: N{:.2}", average);


}