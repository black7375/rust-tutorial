fn main() {
    // == Basics ===============================================================
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<Draw>>, // Trait Object, dynamic dispatch
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw(); // Like duck typing
            }
        }
    }

    // Refactoring - Trait bound
    pub struct Screen<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> Screen<T>
    where T: Draw {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // == Impliments ===========================================================
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }


    // Other Creates
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // code to actually draw a select box
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // == Object Safe ==========================================================
    // Not safe
    // - Return type is `Self`
    // - Generic type arguments

    // Not safe - sample
    pub trait Clone {
        fn clone(&self) -> Self;
    }

    pub struct Screen {
        pub components: Vec<Box<Clone>>,
    }
}
