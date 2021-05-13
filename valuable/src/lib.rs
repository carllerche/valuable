//! Valuable provides object-safe value inspection. Use cases include passing
//! structured data to trait objects and object-safe serialization.
//!
//! # Getting started
//!
//! TODO
//!
//! # Design
//!
//! TODO

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod enumerable;
pub use enumerable::{EnumDef, Enumerable, Variant, VariantDef};

pub mod field;

mod listable;
pub use listable::Listable;

mod mappable;
pub use mappable::Mappable;

mod named_values;
pub use named_values::NamedValues;

mod slice;
pub use slice::Slice;

mod structable;
pub use structable::{StructDef, Structable};

mod valuable;
pub use crate::valuable::Valuable;

mod value;
pub use value::Value;

mod visit;
pub use visit::Visit;

#[cfg(feature = "derive")]
pub use valuable_derive::*;
