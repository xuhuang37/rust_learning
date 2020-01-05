fn main() {
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {}", s2);

    // s1.push('l');
    // println!("s1 is {}", s1);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("s3 is {}",s3);
    // println!("s2 is {}",s2);
    // println!("s1 is {}",s1);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);

    // let s1 = String::from("hello");
    // let h = s1[0];

    // let len = String::from("Hola").len();
    // let len = String::from("Здравствуйте").len();
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    // let s = &hello[0..1];
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


}
