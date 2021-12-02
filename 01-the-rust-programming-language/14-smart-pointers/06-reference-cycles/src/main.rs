fn main() {
    //== Basics ================================================================
    //-- Definition ------------------------------------------------------------
    use std::rc::Rc;
    use std::cell::RefCell;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        // helper fn
        fn new(num: i32) -> Rc<List> {
            Rc::new(Cons(num, RefCell::new(Rc::new(Nil))))
        }
        fn append(num: i32, list: &Rc<List>) -> Rc<List> {
            Rc::new(Cons(num, RefCell::new(Rc::clone(list))))
        }

        // method
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match *self {
                Cons(_, ref item) => Some(item),
                Nil => None,
            }
        }
    }

    //-- Implement -------------------------------------------------------------
    let a = List::new(5);
    println!("a initial rc count = {}", Rc::strong_count(&a)); // 5
    println!("a next item = {:?}", a.tail()); // Some(RefCell { value: Nil })

    let b = List::append(10, &a);

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail()); // Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    // Change a's Nil -> &b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail()); // println!("a next item = {:?}", a.tail());

    //== Weak ==================================================================
    //-- Definition ------------------------------------------------------------
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    //-- Implement -------------------------------------------------------------
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None

    //== Strong count / Weak count =============================================
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
