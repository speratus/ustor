use std::collections::HashMap;
use super::err;

pub mod db {
    use std::collections::HashMap;
    use crate::err::err::UDisErr;
    use crate::err::err::UDisErr::KeyExists;
    use crate::types::types::Insertable;


    pub trait Udb {
        fn insert_or_update(&mut self, key: String, val: Box<dyn Insertable>) -> Option<dyn Insertable>;

        fn remove(&mut self, key: String);

        fn get(&self, key: String) -> Option<dyn Insertable>;
    }


    pub struct UDbImpl {
        dict: HashMap<String, dyn Insertable>,
    }

    impl UDb for UDbImpl {

        fn insert_or_update(&mut self, key: String, val: Box<dyn Insertable>) -> Option<dyn Insertable> {
            self.dict.insert(key, val)
        }

        fn remove(&mut self, key: String) {
            self.dict.remove(key.as_str());
        }

        fn get(&self, key: String) -> Option<dyn Insertable> {
            self.dict.get(key.as_str())
        }
    }
}
