mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64) -> f64 { /* ... */ }
    pub fn cos(f: f64) -> f64 { /* ... */ }
    pub fn tan(f: f64) -> f64 { /* ... */ }
}

println!("{}", math::cos(45.0));

// Declare a private struct
struct Foo;

// Declare a public struct with a private field
pub struct Bar {
    field: i32,
}

// Declare a public enum with two public variants
pub enum State {
    PubliclyAccessibleVariant,
    PubliclyAccessibleVariant2,
}
