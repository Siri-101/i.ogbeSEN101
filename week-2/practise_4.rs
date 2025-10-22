fn main(){
    let p:f64 = 1000.00;
    let r:f64 = 1.0;
    let t:f64 = 2.0;
    
    //simple interest

    let a = p * (1.0 + (r/100.0 )) * t;
    println!("Amount is {:.2 }", a);
    let si = a - p;

    println!("simple interest is {:.2}", si);
}