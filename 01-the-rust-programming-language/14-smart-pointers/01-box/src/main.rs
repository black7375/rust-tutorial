
fn main() {
    //== Basics ================================================================
    // - Can't known size at compile time
    // - Large data move ownership, don't want to copy
    // - Specific trait's implement type

    let b = Box::new(5);
    println!("b = {}", b);

    //== Recursive type ========================================================
    // lisp's cons list
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};
    let list = Cons(1,
       Box::new(Cons(2,
           Box::new(Cons(3,
               Box::new(Nil))))));
}
