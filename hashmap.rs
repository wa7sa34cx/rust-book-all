use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    scores.insert("Yellow", 10);
    scores.insert("Blue", 20);

    println!("{:?}", scores);

    // ----

    let teams = vec!["Yellow", "Blue"];
    let start_scores = vec![10, 20];

    let scores: HashMap<_, _> = 
        teams.into_iter().zip(start_scores.into_iter()).collect();

    println!("{:?}", scores);
    
    // ----

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", scores);

    // ----

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_blue = String::from("Blue");
    let score_blue = scores.get(&team_blue);

    println!("{:?}", score_blue);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // ---- rewrite value

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // ---- insert if doesn't exist

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // counts how many times each word appears in some text

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

     println!("{:?}", map);
}
