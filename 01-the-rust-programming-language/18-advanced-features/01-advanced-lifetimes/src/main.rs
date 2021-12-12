fn main() {
    // == Subtyping ============================================================
    // Error
    struct Context(&str);

    struct Parser {
        context: &Context,
    }

    impl Parser {
        fn parse(&self) -> Result<(), &str> {
            Err(&self.context.0[1..])
        }
    }

    // Works
    struct Context<'a>(&'a str);

    struct Parser<'a> {
        context: &'a Context<'a>,
    }

    impl<'a> Parser<'a> {
        fn parse(&self) -> Result<(), &str> { // fn parse<'a>(&'a self) -> Result<(), &'a str> {
            Err(&self.context.0[1..])
        }
    }

    // But this code is Error
    // Paser, context need more life then fn
    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }

    // Works All
    struct Context<'s>(&'s str);

    struct Parser<'c, 's: 'c> {    // 's must live loger than 'c
        context: &'c Context<'s>,  // But compiler can't known releation
    }

    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }
    }

    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }

    // == Bound ================================================================
    struct Ref<'a, T: 'a>(&'a T); // T live as 'a
    struct StaticRef<T: 'static>(&'static T);

    // == Inference ============================================================
    trait Red { }

    struct Ball<'a> {
        diameter: &'a i32,
    }

    impl<'a> Red for Ball<'a> { } // Trait object with lifetime

    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<Red>;
}
