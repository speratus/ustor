pub mod store;
pub mod commands;

pub trait Data {}

macro_rules! type_impl {
    ($type:ty) => {
        impl Data for $type {}
    };
    ($first:ty, $($rest:ty),+) => {
        type_impl!($first);
        type_impl!($($rest),+);
    }
}

type_impl!(bool, i8, u8, i32, u32, i128, u128, String);

pub type Value = Box<dyn Data>;