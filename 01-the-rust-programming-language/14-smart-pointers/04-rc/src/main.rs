fn main() {
    //== Basics ================================================================
    // Not works!!
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};

    let a = Cons(5,
                 Box::new(Cons(10,
                               Box::new(Nil))));
    let b = Cons(3, Box::new(a)); // value moved here
    let c = Cons(4, Box::new(a)); // value used here after move


    // Use Rc<T>
    // Rc<T> only works single thread
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // if use a.clone(): deep copy
    // Rc::clone(&a): Just +1 count

    //== Counting ==============================================================
    // When scoped out, Drop trait's implement is count -1
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));          // 1
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));          // 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));      // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}
