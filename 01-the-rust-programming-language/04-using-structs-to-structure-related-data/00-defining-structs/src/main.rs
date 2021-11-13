struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    //== Basics ================================================================
    // New instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Assignment
    user1.email = String::from("anotheremail@example.com");

    //== Field init shorthand ==================================================
    fn build_user1(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Same
    fn build_user2(email: String, username: String) -> User {
        User {
            email,    // Field init shorthand, Same as `email: email`
            username, // Field init shorthand, Same as `username: username`
            active: true,
            sign_in_count: 1,
        }
    }

    //== Struct update syntax ==================================================
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // same
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    //== Tuple Struct ==========================================================
    struct Color(i32, i32, i32); // Color, Point is other type.
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    //== Struct's ownership ====================================================
    struct User2 {
        username: &str,
        email: &str,
        sign_in_count: u64,
        active: bool,
    }

    // &str don't have lifetime
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
