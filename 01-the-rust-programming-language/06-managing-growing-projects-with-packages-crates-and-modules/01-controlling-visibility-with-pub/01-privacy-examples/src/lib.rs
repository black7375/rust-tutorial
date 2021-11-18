mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {
            // https://users.rust-lang.org/t/error-e0433-failed-to-resolve-could-not-find-outermost-in-root/23220
            crate::outermost::middle_secret_function();
        }
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
