fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 3, 4];
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(3);
    // v.push(1);
    // v.push(2);
    // v.push(4);
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];

    // println!("The third element is {}", third);
    // match v.get(2) {
    //     Some(third) => println!("The thrid element is {}", third),
    //     None => println!("There is no third element."),
    // }
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    v.push(6);
    // let _does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
    // let first = &v[0];

    // v.push(6);
    // println!("The first element is: {}", first);
    let mut v1 = vec![100,32,57];
    for i in &mut v1 {
        *i += 50;
        println!("{}", i);
    }
    
    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}




