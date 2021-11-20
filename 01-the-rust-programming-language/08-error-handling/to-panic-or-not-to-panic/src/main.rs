fn main() {
    //== Guide =================================================================
    /* GUIDE!!
    1. Result<T, E> Type:
       * Provides options whether recovery is possible or not.
       * When you expect to be in a bad state even if you write good code

    2. Panic!:
       * At:  Examples, Prototype Code, Test
       * Use: unwrap, expect method
       * When you are sure
         If you're sure it has more information than the compiler,
         and you're always type `Ok`.

         Example (Hard Coded)
         ```
         use std::net::IpAddr;
         let home = "127.0.0.1".parse::<IpAddr>().unwrap(); // Always Ok
         ```
       * Other Guides
         - The bad state is not something that’s expected to happen occasionally.
         - Your code after this point needs to rely on not being in this bad state.
         - There’s not a good way to encode this information in the types you use.
     */

    //== Custom Type ===========================================================
    loop {
        // snip

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // snip
        }
    }

    // Refactoring
    pub struct Guess {
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
}
