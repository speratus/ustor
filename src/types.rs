pub mod types {
    use std::hash::Hash;

    pub trait Udb {
        fn insert_or_update<T>(&mut self, key: String, val: T) -> Option<T>;

        fn remove(&mut self, key: String);

        fn get<T>(&self, key: String) -> Option<T>;

        fn key_exists(&self, key: String) -> bool;
    }

}