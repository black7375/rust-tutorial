use std::collections::HashMap;

fn main() {
    //== Generate ==============================================================
    // Basic
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),   10);
    scores.insert(String::from("Yellow"), 50);

    // From vec
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //== Ownership =============================================================
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Now field_name, field_value: can't use!!

    //== Access ================================================================
    // Get
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterate
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //== Update ================================================================
    // Overwrite
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // {"Blue": 25}

    // Block overwrite
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}

    // Based on before value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // return `&mut V`
        *count += 1;
    }

    println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}
}
