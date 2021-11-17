fn main() {
    //== Enum Basics ===========================================================
    enum IpAddrKind {
        V4, // Variants
        V6, // Variants
    }

    let four = IpAddrKind::V4;
    let six  = IpAddrKind::V6;

    //== Enum with Structs =====================================================
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    // instance
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //== Enum with Tuple =======================================================
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    //== Refactoring ===========================================================
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    //== Compare to structs ====================================================
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage; // Unit Struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // Tuple Struct
    struct ChangeColorMessage(i32, i32, i32); // Tuple Struct

    //== Method ================================================================
    impl Message {
        fn call(&self) {
            // method...
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    //== Optional ==============================================================
    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    //== Optional More =========================================================
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // Not work. Option<i8> is other type.
}
