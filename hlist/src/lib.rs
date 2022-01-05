//! A heterogeneous list, similar to a tuple, but inductively defined.
//!
//! By encoding more information into types, we can let the type-checker enforce properties on our behalf.

#![feature(generic_associated_types)]

pub mod boolean;
pub mod hlist;
pub mod nat;

pub use hlist::{HCons, HList, HNil};
