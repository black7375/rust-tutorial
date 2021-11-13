fn main() {
    //== Get first word space index ============================================
    let mut s = String::from("hello world");

    let word = first_word(&s); // word is 5

    s.clear(); // makes to ""

    // word have 5, but can't use String
    // word is not vaild

    // first_word
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); // byte array

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    //== String Slice ==========================================================
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];


    // Other samples - 1
    let slice = &s[0..2]; // Same to ..2
    let slice = &s[..2];  // Same to 0..2

    // Other samples - 2
    let len = s.len();
    let slice = &[3..len]; // Same to 3..
    let slcie = &[3..];    // Same to 3..len

    // Other samples - 3
    let slice = &s[0..len]; // Same to ..
    let slice = &s[..];     // Same to 0..len

    //== Rewrite with String Slice =============================================
    let mut s = String::from("hello world");

    let word = first_word_slice(&s);
    s.clear(); // Will be compile error!!

    fn first_word_slice(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    //== More String Slice =====================================================
    // `String`'s slice
    let s = String::from("hello world");
    let word = first_word_slice_more(&s[..]);

    // `&str`(Literal)'s slice
    let s = "hello world"; // Literal is string slice (s: &str)
    let word = first_word_slice_more(&s[..]);
    let word = first_word_slice_more(s);      // Works: string literal is slice!!

    fn first_word_slice_more(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    //== The Other Slices ======================================================
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];   // array slice, &[i32]
}
