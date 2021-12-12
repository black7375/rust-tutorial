fn main() {
    // == Type Alias ===========================================================
    // -- Basics ---------------------------------------------------------------
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // -- Example --------------------------------------------------------------
    let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<Fn() + Send + 'static> {
        // --snip--
    }

    // Refactoring
    type Thunk = Box<Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }

    // -- Example --------------------------------------------------------------
    use std::io::Error;
    use std::fmt;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }

    // Refactoring
    type Result<T> = Result<T, std::io::Error>;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
    }

    // == Never Type ===========================================================
    // Empty type, Not return type
    fn bar() -> ! {
        // --snip--
    }

    // -- Continue -------------------------------------------------------------
    // Works
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue, // Never
    };

    // Not works - Need same return type
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "hello",
    }

    // -- Panic! ---------------------------------------------------------------
    impl<T> Option<T> {
        pub fn unwrap(self) -> T {
            match self {
                Some(val) => val,
                None => panic!("called `Option::unwrap()` on a `None` value"), // Never
            }
        }
    }

    // -- Eternal Loop ---------------------------------------------------------
    // Don't break loop
    loop {
        print!("and ever ");
    }

    // == Dynamic Sized Type ===================================================
    // Static size
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";

    // Generic's size
    fn generic<T>(t: T) {
        // --snip--
    }
    fn generic<T: Sized>(t: T) {   // Known size at compile time
        // --snip--
    }
    fn generic<T: ?Sized>(t: &T) { // Unknown(dynamic) size, needs reference
        // --snip--
    }
}
