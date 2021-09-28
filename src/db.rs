use std::collections::HashMap;
use super::err;

pub mod db {
    use std::collections::HashMap;
    use crate::err::err::UDisErr;
    use crate::err::err::UDisErr::KeyExists;

    type Result<T> = std::Result<T, UDisErr<String>>;

    pub struct UDb {
        dict: HashMap<String, String>,
    }

    impl UDb {

        fn insert_or_update(&mut self, key: String, val: String) -> Option<String> {
            self.dict.insert(key, val)
        }

        fn remove(&mut self, key: String) {
            self.dict.remove(key.as_str());
        }

        fn get(&self, key: String) -> Option<&String> {
            self.dict.get(key.as_str())
        }
    }
}
