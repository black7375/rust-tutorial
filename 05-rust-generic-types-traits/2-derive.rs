use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    println!("{}", p1);
    println!("{:?}", p1);

    if p1 == p2 {
        println!("equal!");
    } else {
        println!("not equal!");
    }
}

