fn main() {
    // == Array Define =========================================================
    // Declare array, initialize all values, compiler infers length = 7
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    // Declare array, first value = "0", length = 5
    let bytes = [0; 5];

    // == Array Indexing =======================================================
    // Set first day of week
    let first  = days[0];

    // Set second day of week
    let second = days[1];

    // == Vector ===============================================================
    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    // Declare vector, first value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    // == Vector new ===========================================================
    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);

    // Access vector with out-of-bounds index
    let beyond = index_vec[10];
    println!("{}", beyond);

    // == HashMap ==============================================================
    // Use the HashMap definition from Rust standard library
    use std::collections::HashMap;

    // Declare variable as mutable so we can change keys and values
    // Both keys and values use the String type
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert("Ancient Roman History".to_string(), "Very accurate.".to_string());
    reviews.insert("Cooking with Rhubarb".to_string(), "Sweet recipes.".to_string());
    reviews.insert("Programming in Rust".to_string(), "Great examples.".to_string());

    // Look for specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}
