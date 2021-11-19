fn main() {
    // str: UTF-8, string slice, literal
    // String: UTF-8, std

    //== Generate ==============================================================
    // empty string
    let mut s = String::new();

    // str -> String
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string(); // // the method also works on a literal directly

    let s = String::from("initial contents");

    // UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //== Update ================================================================
    // Add
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str don't borrow
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2); // Works!!, Because push_str(&str)

    // push: char
    let mut s = String::from("lo");
    s.push('l');

    // Move
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1: moved, can't use
    // + operator: fn add(self, s: &str) -> String {

    // Multiple
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    // format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // like println!

    //== Indexing ==============================================================
    let s1 = String::from("hello");
    let h = s1[0]; // Error!!!

    // String: Vec<u8> wrapper
    let len = String::from("Hola").len(); // 4 byte
    let len = String::from("안녕하세요").len(); // 15 byte

    // Can't use index
    let hello  = String::from("안녕하세요");
    let answer = &hello[0]; // Error!!

    // Byte, Grapheme cluster
    // Hindi: “नमस्ते”
    //     Vec<u8>: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    //               224, 165, 135]
    //     char:    ['न', 'म', 'स', '्', 'त', 'े']
    //     cluster: ["न", "म", "स्", "ते"]


    //== Slicing ===============================================================
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Зд
    let s = &hello[0..1]; // panic!

    //== Iterate ===============================================================
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    /* Print
      न
      म
      स
      ्
      त
      े
     */


    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    /* Print (18 byte)
    224
    164
    168
    224
    ..etc
     */
}
