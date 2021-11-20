fn main() {
    //== Basics ================================================================
    /* panic! do unwinding

    if not,
    ```toml
    # Cargo.toml
    [profile.release]
    panic = 'abort'
    ```
     */
    panic!("crash and burn"); // Make panic & Error

    //== Backtrace =============================================================
    let v = vec![1, 2, 3];
    v[99]; // Buffer overread, panic!

    // Try `RUST_BACKTRACE=1 cargo run` for backtrace
}
