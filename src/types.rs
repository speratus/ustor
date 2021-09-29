pub mod types {
    use std::hash::Hash;

    pub trait Insertable: Hash + Eq {}

}