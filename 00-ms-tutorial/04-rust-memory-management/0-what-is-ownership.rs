fn process(input: u32) {}
fn process2(s: String) {}

fn main() {
    // == Binding ==============================================================
    {
        let mascot = String::from("ferris");
        let ferris = mascot;
        println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
    }

    // == Ownership ============================================================
    let s = String::from("Hello, world!");
    process2(s.clone()); // Passing another value, cloned from `s`.
    process2(s); // s was never moved and so it can still be used.
}

fn caller() {
    let n = 1u32;
    process(n); // Ownership of the number in `n` copied into `process`
    process(n); // `n` can be used again because it wasn't moved, it was copied.
}
