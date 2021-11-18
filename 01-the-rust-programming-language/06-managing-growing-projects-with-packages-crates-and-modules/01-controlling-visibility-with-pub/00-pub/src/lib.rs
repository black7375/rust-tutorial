// communicator
//  ├── client
//  └── network
//      └── server
pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
        network::connect();
        network::server::connect();
    }
}
