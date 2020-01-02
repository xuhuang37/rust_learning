fn main() {
    let s = String::from("hello world");
    let s_literal = "hello world";
    // let hello = &s[..];
    // let world = &s[6..11];
    // let tiny = &world[..2];
    // println!("hello:{},world:{},tiny:{}",hello,world,tiny);
    
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    let s_first = first_word(&s);
    let s_first_literal = first_word(s_literal);

    // s.clear();
    println!("{}", s_first);
    println!("{}", s_first_literal);

}
