fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn change(text: &mut String) {
    text.push_str(", world");
}

fn main() {
    let mut greeting = String::from("Hello");

    // == Reference ============================================================
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`
    println!("Greeting: {}", &greeting_reference);

    // == Function Reference ===================================================
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

    // == Borrow & Mutable =====================================================
    change(&mut greeting);
    println!("Greeting: {}", greeting); // We can still use `greeting`
}
