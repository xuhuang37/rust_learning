// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

fn main() {
    // let r;
    // let x = 5;
    // r = &x;
    // println!("r: {}", r);
    //Generic Lifetimes in Functions

    let string1 = String::from("abcd");

    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("result: {}", result);
    }


    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // Lifetime Annotations is Struct Definitions

    // let novel = String::from("Call me Ishmael. Some years ago...");
    // let first_sentence = novel.split('.')
    // .next()
    // .expect("Could not find a '.'");

    // let i = ImportantExcerpt { part: first_sentence };

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // let result = String::from("really long string");
    // result.as_str()
    if x.len() > y.len() {
        x
    }else {
        y
    }
}
