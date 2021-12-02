fn main() {
    //== Basics ================================================================
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    /* Result
      CustomSmartPointers created.
      Dropping CustomSmartPointer with data `other stuff`!
      Dropping CustomSmartPointer with data `my stuff`!
     */

    //== Early Drop ============================================================
    // Not works
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    c.drop(); // explicit destructor calls not allowed, double free
    println!("CustomSmartPointer dropped before the end of main.");

    // Use std::mem::drop
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c); // Get ownership, for only 1 call
    println!("CustomSmartPointer dropped before the end of main.");
    /* Result
      CustomSmartPointers created.
      Dropping CustomSmartPointer with data `some data`!
      CustomSmartPointer dropped before the end of main.
     */
}
