fn main() {
    // == Raw pointer ==========================================================
    // *const T or *mut T
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Use
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Can't known about vaild
    let address = 0x012345usize;
    let r = address as *const i32;

    // == Unsafe Function or Method ============================================
    // -- Basics ---------------------------------------------------------------
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // -- Split at mut ------------------------------------------------
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Not works
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();

        assert!(mid <= len);

        (&mut slice[..mid], // Error
         &mut slice[mid..]) // No overlap, but ownership issues
    }

    // Works
    use std::slice;
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // *mut i32

        assert!(mid <= len);

        unsafe {
            (slice::from_raw_parts_mut(ptr, mid),
             slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }
    }

    // Crash sample - UB(Undefined Behaviour)
    use std::slice;
    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };

    // == Extern ===============================================================
    extern "C" { // C's abs fn
        fn abs(input: i32) -> i32;
    }

    fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    // Call from other lang
    #[no_mangle] // Don't change fn name
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // == Static variable ======================================================
    // Basic
    static HELLO_WORLD: &str = "Hello, world!"; // &'static str

    fn main() {
        println!("name is: {}", HELLO_WORLD);
    }

    // Mutble
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    fn main() {
        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }


    // == Unsafe Trait =========================================================
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}
