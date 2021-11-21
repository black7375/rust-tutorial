extern crate adder;

/* Only run this file

$ cargo test --test integration_test
 */

// Test's Common module
mod common;

// Don't need #[cfg(test)]
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}


/* Warning!!
  Can't use only exist `src/main.rs`

 */
