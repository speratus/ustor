pub mod err {
    pub enum UDisErr<T> {
        KeyExists(T),
    }
}