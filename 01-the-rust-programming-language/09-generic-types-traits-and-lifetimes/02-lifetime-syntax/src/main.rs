fn main() {
    //== Dangling ==============================================================
    // Works well
    {
        let x = 5;            // -----+-- 'b
                              //      |
        let r = &x;           // --+--+-- 'a
                              //   |  |
        println!("r: {}", r); //   |  |
                              // --+  |
    }                         // -----+

    // Dangling Reference
    {
        let r;         // -------+-- 'a
                       //        |
        {              //        |
            let x = 5; // -+-----+-- 'b
            r = &x;    //  |     |
        }              // -+     |
                       //        |
        println!("r: {}", r); // |
                       //        |
                       // -------+
    }

    // The other lifetime Error
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    //== Basics ================================================================
    /* Syntax
     * &i32        // a reference
     * &'a i32     // a reference with an explicit lifetime
     * &'a mut i32 // a mutable reference with an explicit lifetime
     */

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Works!!
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Not works - Scoped out
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); // Lifetime

    //== More ==================================================================
    // works
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // Not works - Return lifetime not related to parameters
    fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }

    //== Struct ================================================================
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    //== Elision Rules =========================================================
 // fn first_word<'a>(s: &'a str) -> &'a str {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }


    /* Default Rules
    fn foo<'a>(x: &'a i32) => fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
    fn foo<'a>(x: &'a i32) -> &'a i32
    fn foo<'a>(&'a self, x: i32) -> &a
     */

    // With Method
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    //== Static ================================================================
    let s: &'static str = "I have a static lifetime.";

    //== Multiple ==============================================================
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
