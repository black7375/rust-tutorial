fn main() {
    //== Basics ================================================================
    // Traits is like other languages' interface
    pub trait Summarizable {
        fn summary(&self) -> String;
    }

    // Implements Sample 1
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summarizable for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    // Implements Sample 2
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summary());

    //== Orphan Rule ===========================================================
    extern crate aggregator;

    use aggregator::Summarizable;

    struct WeatherForecast {
        high_temp: f64,
        low_temp: f64,
        chance_of_precipitation: f64,
    }

    impl Summarizable for WeatherForecast {
        fn summary(&self) -> String {
            format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
                    self.chance_of_precipitation)
        }
    }

    //== Default Implements ====================================================
    //-- Simple Example --------------------------------------------------------
    pub trait Summarizable {
        fn summary(&self) -> String {
            String::from("(Read more...)")
        }
    }

    impl Summarizable for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
    };

    println!("New article available! {}", article.summary());

    //-- Other Method ----------------------------------------------------------
    pub trait Summarizable {
        fn author_summary(&self) -> String;

        fn summary(&self) -> String {
            format!("(Read more from {}...)", self.author_summary())
        }
    }

    impl Summarizable for Tweet {
        fn author_summary(&self) -> String {
            format!("@{}", self.username)
        }
    }


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    //== Bound =================================================================
    pub fn notify<T: Summarizable>(item: T) {
        println!("Breaking news! {}", item.summary());
    }

    //-- Where -----------------------------------------------------------------
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    }
    fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
    {
    }

    //-- Example ---------------------------------------------------------------
    use std::cmp::PartialOrd;

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }


    // Impl
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);

    // Other Solutions
    use std::cmp::PartialOrd;

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let result = largest(&numbers);
    println!("The largest char is {}", result);
}
