use std::hash::Hash;

use crate::data_types::Data;

pub trait Store<K: Eq + Hash> {

    fn put<V: Data>(key: K, value: V) -> bool;

    fn get<V: Data>(key: K) -> V;

    fn update<V: Data>(key: K, value: V) -> bool;

    fn has(key: K) -> bool;

    fn size() -> usize;

}