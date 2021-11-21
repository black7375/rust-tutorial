pub fn add_two(a: i32) -> i32 { // public
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 { // private
    a + b
}

#[cfg(test)] // Not compile at `cargo build`, only `cargo test`
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
