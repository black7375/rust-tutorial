fn main() {
    //== Refutablity ===========================================================
    let x = 5;                       // irrefutable, can't failed
    let Some(x) = some_option_value; // refutable, can failed(None)

    // solution
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // Error
    if let x = 5 {                   // irrefutable
        println!("{}", x);
    };
}
