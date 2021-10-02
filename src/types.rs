pub mod types {
    use std::hash::Hash;
    use std::net::{SocketAddr, TcpStream};

    pub trait Udb {
        fn insert_or_update<T>(&mut self, key: String, val: T) -> Option<T>;

        fn remove(&mut self, key: String);

        fn get<T>(&self, key: String) -> Option<T>;

        fn key_exists(&self, key: String) -> bool;
    }

    pub trait Client {
        fn get_remote_addr(&self) -> &SocketAddr;

        fn get_stream(&self) -> &TcpStream;

        fn get_stream_mutable(&self) -> &mut TcpStream;

        fn subscribed_channels(&self) -> &Vec<String>;
    }

}