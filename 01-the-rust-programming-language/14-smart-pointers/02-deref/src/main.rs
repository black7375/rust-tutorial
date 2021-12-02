fn main() {
    //== Basics ================================================================
    // Deref trait: Dereference operator `*`'s behavior customize
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //== My SmartPointer =======================================================
    // Define Struct
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // Implement Deref
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    // Use
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref())

    //== Deref coercion ========================================================
    // Compile time analysis!!
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Deref coercion: &MyBox<String> -> &String
               // Deref coercion: &String -> &str

    // If don't have deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    //== Deref coercion with mut ===============================================
    // T: Deref<Target=U>    => &T     -> &U
    // T: DerefMut<Target=U> => &mut T -> &mut U
    // T: Deref<Target=U>    => &mut T -> &U
}
