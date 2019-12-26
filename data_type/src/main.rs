fn main() {
    let guess:u32 = "42".parse().expect("Not a number!");


    let tup:(u32,f64,u8) = (500,6.4,1);
    let tup = (500,6.4,1);

    let (x,y,z) = tup;
    println!("{}",y);
}
