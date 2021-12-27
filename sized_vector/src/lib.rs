//! A sized-vector where the length information is encoded into the type-level.
//!
//! By encoding more information into types, we can let the type-checker enforce properties on our behalf.

#![feature(generic_associated_types)]

pub mod ivec;
pub mod nat;
