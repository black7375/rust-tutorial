//== Module ====================================================================
#[cfg(test)]
mod tests1 {
    #[test]
    fn it_works() {
        network::connect();
        client::connect();
    }

    mod network {
        pub fn connect() {
        }
    }

    mod client {
        pub fn connect() {
        }
    }
}

//== Nested Module =============================================================
#[cfg(test)]
mod tests2 {
    #[test]
    fn it_works() {
        network::connect();
        network::client::connect();
    }

    mod network {
        pub fn connect() {
        }

        pub mod client {
            pub fn connect() {
            }
        }
    }
}

//== Other File Module =========================================================
// communicator
//  ├── client
//  └── network
//      └── server
#[cfg(test)]
mod tests3 {
    mod client;
    mod network;

    #[test]
    fn it_works() {
        client::connect();
        network::connect();
        network::server::connect();
    }
}
