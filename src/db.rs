use std::collections::HashMap;
use super::err;

pub mod db {
    use std::collections::HashMap;
    use crate::err::err::UDisErr;
    use crate::err::err::UDisErr::KeyExists;
    use crate::types::types::Insertable;


    pub struct UDbImpl {
        dict: HashMap<String, dyn Insertable>,
    }

    impl UDb for UDbImpl {

        fn insert_or_update<T>(&mut self, key: String, val: T) -> Option<T> {
            self.dict.insert(key, val)
        }

        fn remove(&mut self, key: String) {
            self.dict.remove(key.as_str());
        }

        fn get<T>(&self, key: String) -> Option<T> {
            self.dict.get(key.as_str())
        }
    }
}
