fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // == Pick with optional ===================================================
    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // == match ================================================================
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    // == match / if ===========================================================
    let a_number: Option<u8> = Some(7);

    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {},
    }
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }

    // == unwrap / expect ======================================================
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!


    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    let b: Option<&str> = None;
    b.expect("fruits are healthy"); // panics with `fruits are healthy`

    // Better
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}
