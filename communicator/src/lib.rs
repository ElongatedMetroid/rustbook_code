// declares client module here but rust looks for it in a different location
// in this case src/client.rs since the modules name is client
pub mod client;

// define a module named network that contains the function called connect
pub mod network;

// you can have multiple modules in one file

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
