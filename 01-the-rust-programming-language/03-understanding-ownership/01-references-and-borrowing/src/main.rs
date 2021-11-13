
fn main() {
    //== Refer - Don't have ownership ==========================================
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        s.len()
    } // s: goes out of scope, but refer does not have ownership.
      //    Nothing happens

    //== Refer - Change with &mut = ============================================
    let mut s2 = String::from("hello");

    change(&mut s2); // Not works just `&s`

    // Not works just `&String`
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    //== Refer - Can't use multiple &mut =======================================
    let mut s3 = String::from("hello");

    // Not works
    let r1 = &mut s3;
    let r2 = &mut s3;

    // Works
    {
        let r1 = &mut s3;
    }
    let r2 = &mut s3;

    // Not works
    let r1 = &s3; // No problem
    let r2 = &s3; // No problem
    let r3 = &mut s; // Problem

    //== Dangling References ===================================================
    // Not works, because of lifetime
    let reference_to_nothing = dangle();

    // Works
    let owner = no_dangle();

    fn dangle() -> &String {
        let s = String::from("hello");

        &s
    }

    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }
}
