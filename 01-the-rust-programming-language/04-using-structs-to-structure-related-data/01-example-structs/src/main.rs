fn main() {
    //== Basics ================================================================
    let length1 = 50;
    let width1 = 30;

    fn area(length: u32, width: u32) -> u32 {
        length * width
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );

    //== Refactoring with Tuple ================================================
    let rect1 = (50, 30);

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    //== Refactoring with Struct ===============================================
    struct Rectangle {
        length: u32,
        width: u32,
    }

    let rect1 = Rectangle { length: 50, width: 30 };

    n area(rectangle: &Rectangle) -> u32 {
        rectangle.length * rectangle.width
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    //== Derived trait =========================================================
    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }

    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {}",    rect1); // Not works
    println!("rect1 is {:?}",  rect1); // rect1 is Rectangle { length: 50, width: 30 }
    println!("rect1 is {:#?}", rect1); // with formatting
}
