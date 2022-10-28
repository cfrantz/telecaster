#![feature(min_specialization)]

pub use std::any::TypeId;
mod type_id;

pub use type_id::{GetTypeId, GetSelfId};
