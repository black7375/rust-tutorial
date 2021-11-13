fn main() {
    //== Scope =================================================================
    let s = "hello";
    {
                                              // s: Not vaild, not yet decalred
        let s = "hello";                      // s: vaild

        // do stuff with s
    }                                         // s: No longer valid, scope is now over

    //== String Type ===========================================================
    let mut s = String::from("hello");        // Heap
    s.push_str(", world!");                   // append a literal str
    println!("{}", s);

    //== Memory / Allocate =====================================================
    let x = 5;
    let y = x;                                // x: Clone
    println!("x: {}, y: {}", x, y);           // Ok

    let s1 = String::from("hello");
    let s2 = s1;                              // s1: Move
    println!("{}, world!", s1);               // Error

    let s1 = String::from("hello");
    let s2 = s1.clone();                      // s1: Clone
    println!("{}, world!", s1);               // Ok

    //== Ownership / Function ==================================================
    {
        let s = String::from("hello");        // s: comes into scope

        takes_ownership(s);                   // s: No longer vaild, move into the function
        let x = 5;                            // x: comes into scope

        makes_copy(x);                        // x: Valid, Move into the function
                                              //    Because, i32 is clone.

    }                                         // x: goes out of scope, then s.
                                              //    But s's value was moved.

    fn takes_ownership(some_string: String) { // some_string: comes into scope
        println!("{}", some_string);
    }                                         // some_string: goes out of scope and `drop` is called.
                                              //              memory is freed.

    fn makes_copy(some_integer: i32) {        // some_integer: comes into scope
        println!("{}", some_integer);
    }                                         // some_integer: goes out of scope, Nothing special happens.

    //== Return / Scope ========================================================
    {
        let s1 = gives_ownership();           // s1: move out from
                                              //     gives_ownership return value

        let s2 = String::from("hello");       // s2: comes into scope

        let s3 = takes_and_gives_back(s2);    // s2: moved into takes_and_gives_back,
                                              //     also moves its return value into s3

    }                                         // s3: goes out of scope and dropped.
                                              // s2: goes out of scope, but already moved. (Nothing happens)
                                              // s1: goes out of scope and dropped.

    // gives_ownership: will move its return value into the function caller.
    fn gives_ownership() -> String {
        let some_string = String::from("hello"); // some_string: comes into scope

        some_string                              // some_string: is returned,
                                                 //              move out to the calling function
    }

    // takes_and_gives_back: take String and return
    fn takes_and_gives_back(a_string: String) -> String { // a_string: comes into scope

        a_string                                          // a_string is returned,
                                                          // moves out to the calling function
    }


    //-- Multi Value -----------------------------------------------------------
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1); // Destructuring assignment

        println!("The length of '{}' is {}.", s2, len);
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // string's length

        (s, length)
    }
}
