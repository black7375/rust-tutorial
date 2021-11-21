#[cfg(test)]
mod tests {
    //== Print Result ==========================================================
    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {}", a);
        10
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    /* Result - Capture (Default)
    running 2 tests
    test tests::this_test_will_pass ... ok
    test tests::this_test_will_fail ... FAILED

    failures:

    ---- tests::this_test_will_fail stdout ----
    I got the value 8
    thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
    left: `5`,
    right: `10`', src/lib.rs:19:8
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
    tests::this_test_will_fail

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
    */

    /* Result - No Capture
    running 2 tests
    I got the value 4
    I got the value 8
    test tests::this_test_will_pass ... ok
    thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
    left: `5`,
    right: `10`', src/lib.rs:19:8
    note: Run with `RUST_BACKTRACE=1` for a backtrace.
    test tests::this_test_will_fail ... FAILED

    failures:

    failures:
    tests::this_test_will_fail

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
     */
}
