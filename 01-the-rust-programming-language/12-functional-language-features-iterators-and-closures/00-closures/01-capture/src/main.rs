fn main() {
    //== Closure ===============================================================
    // function vs closure
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    // Error closure - Multiple type
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    //== Capture ===============================================================
    let x = 4;
    let y = 4;

    let equal_to_x = |z| z == x;              // Closure  - Capture possible
    fn equal_to_x2(z: i32) -> bool { z == x } // Function - Capture impossible

    assert!(equal_to_x( y)); // works
    assert!(equal_to_x2(y)); // not works

    //== Traits ================================================================
    // FnOnce - Consume captured values. Get ownership. Call once.
    // Fn - Borrow values to immutable.
    // Fnmut - Borrow values to mutable.

    let x = vec![1, 2, 3];
    let y = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;      // Not works
    println!("can't use x here: {:?}", x); // `x` used after move

    assert!(equal_to_x(y));
}
