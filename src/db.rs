use std::collections::HashMap;

use crate::types::Udb;

type AnyBox = Box<_>;

pub struct UDbImpl {
    dict: HashMap<String, AnyBox>,
}

impl UDb for UDbImpl {

    fn insert_or_update<T>(&mut self, key: String, val: T) -> Option<T> {
        self.dict.insert(key, AnyBox::new(val))
    }

    fn remove(&mut self, key: String) {
        self.dict.remove(key.as_str());
    }

    fn get(&self, key: String) -> Option<Anybox> {
        self.dict.get(key.as_str())
    }

    fn key_exists(&self, key: String) -> bool {
        self.dict.keys().filter(|k: &String| {k == key}).collect().len() > 0
    }

}
