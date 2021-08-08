mod authentication;

fn main() {
    let mut user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    //  println!("The password is: {}", user.password_hash); => hash_password is private
    user.set_password("even-more-secret");
}
