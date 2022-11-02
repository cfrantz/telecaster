#![feature(min_specialization)]
#![feature(ptr_metadata)]
#![feature(coerce_unsized)]
#![feature(unsize)]

pub use std::any::TypeId;
mod cast;
mod error;
mod token;
mod type_id;

pub use cast::{Telecaster, TraitObject};
pub use error::Error;
pub use type_id::{GetSelfId, GetTypeId};
pub use token::{TypeToken, BoxToken};
