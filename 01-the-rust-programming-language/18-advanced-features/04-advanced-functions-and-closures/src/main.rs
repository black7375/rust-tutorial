fn main() {
    // == Function Pointer =====================================================
    // -- Basics ---------------------------------------------------------------
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { // fn: function type, Fn: closure trait
        f(arg) + f(arg) // fn type implement Fn, FnMut, FnOnce traits
    }

    fn main() {
        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);
    }

    // -- closure or fn --------------------------------------------------------
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();

    // Same
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();

    // == Return Closure =======================================================
    // Not works (Can't known Sized)
    fn returns_closure() -> Fn(i32) -> i32 {
        |x| x + 1
    }

    // Works
    fn returns_closure() -> Box<Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
