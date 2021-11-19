fn main() {
    //== Init ==================================================================
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    //== Update ================================================================
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //== Scope =================================================================
    {
        let v = vec![1, 2, 3, 4];

        // Do something with v
        // ...
    } // v: goes out of scope, freed here

    //== Read ==================================================================
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];            // Index
    let third: Option<&i32> = v.get(2); // Get

    let does_not_exist = &v[100];       // panic!
    let does_not_exist = v.get(100);    // return `None`

    //== Ownership & Borrow ====================================================
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow occurs here
    v.push(6);         // mutable borrow occurs here

    //== Iterate ===============================================================
    // Immutable
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Mutable
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    //== Enum with Multiple Type ===============================================
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
