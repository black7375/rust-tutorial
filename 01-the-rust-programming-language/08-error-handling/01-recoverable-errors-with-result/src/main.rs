fn main() {
    //== Basics ================================================================
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // Try open file
    use std::fs::File;
    let f = File::open("hello.txt");

    let f = match f {
        // T: std::fs::File
        // E: std::io::Error
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    //== Each Other Error ======================================================
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => { // Match guard
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    //== unwrap & expect =======================================================
    use std::fs::File;

    let f = File::open("hello.txt").unwrap(); // If file not exist, panic!
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // panic! error message

    //== Error Propagation =====================================================
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    //== Error Shortcut `?` ====================================================
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?; // If `Err`, return
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // More refactoring with chaining
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    //== `?` only can use in `Result<T, E>` function ===========================
    use std::fs::File;
    let f = File::open("hello.txt")?; // Error, main fn's result type is `()`
}
