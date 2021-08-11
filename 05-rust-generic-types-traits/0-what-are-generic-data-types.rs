struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    // == Basic Generic ========================================================
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point { x: "high", y: "low" };

    // == Multiple Generic =====================================================
    let integer_and_boolean = Point2 { x: 5, y: false };
    let float_and_string = Point2 { x: 1.0, y: "hey" };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    let both_integer = Point2 { x: 10, y: 30 };
    let both_boolean = Point2 { x: true, y: true };
}
