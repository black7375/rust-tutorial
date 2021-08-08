fn main() {
    println!("Hello, world!");
    goodbye();


    is_divisible_by(12, 4);
    is_divisible_by(13, 5);
    is_divisible_by(14, 0);

    if is_zero(0) {
        println!("The value is zero.");
    }
}

fn goodbye() {
    println!("Goodbye!");
}

fn is_divisible_by(dividend: u32, divisor: u32) {
    // If the divisor is zero, stop execution
    if divisor == 0 {
        println!("\nError! Division by zero is not allowed.");
    } else if dividend % divisor > 0 {
        println!("\n{} % {} has a remainder of {}.", dividend, divisor, (dividend % divisor));
    } else {
        println!("\n{} % {} has no remainder.", dividend, divisor);
    }
}

fn is_divisible_by2(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        println!("\nError! Division by zero is not allowed.");
        // To prevent division by zero, halt execution and return to the caller
        return false;
    } else if dividend % divisor > 0 {
        println!("\n{} % {} has a remainder of {}.", dividend, divisor, (dividend % divisor));
    } else {
        println!("\n{} % {} has no remainder.", dividend, divisor);
    }

    // Create the boolean value and return it to the function caller
    dividend % divisor == 0
}

fn is_zero(input: u8) -> bool {
    if input == 0 {
        return true;
    }
    false
}
