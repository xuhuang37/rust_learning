use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"),10);
    // scores.insert(String::from("Yellow"),50);
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let _scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

    // Hash Maps and Ownership
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // Accessing Values in a hash map

    let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let _score = scores.get(&team_name);

    // Iterate over each key/value pair in a hash map;
    // for (key, value) in &scores {
    //     println!("{}:{}", key, value);
    // }

    // Updating a Hash Map

    // Overwriting a value

    scores.insert(String::from("Yellow"), 22);

    println!("{:?}", scores);

    //Only Inserting a Value if the Key Has No Value
    scores.entry(String::from("Yellow")).or_insert(50);

    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
