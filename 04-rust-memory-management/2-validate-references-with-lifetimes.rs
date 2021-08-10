fn main() {
    let x;                // ---------+-- 'a
    {                     //          |
        let y = 42;       // -+-- 'b  |
        x = &y;           //  |       |      We store a reference to `y` in `x` but `y` is about to be dropped.
    }                     // -+       |
    println!("x: {}", x); //          |      `x` refers to `y` but `y has been dropped!
}
