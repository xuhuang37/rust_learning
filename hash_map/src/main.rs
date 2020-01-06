use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"),10);
    // scores.insert(String::from("Yellow"),50);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let _scores:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
}
