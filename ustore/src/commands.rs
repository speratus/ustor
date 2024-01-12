use std::hash::Hash;

use crate::Value;

pub enum CommandType {
    Create(Value),
    Retrieve,
    Update(Value),
    Delete
}

pub struct Command<K: Eq + Hash> {
    key: K,
    command: CommandType,
}