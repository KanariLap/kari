#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate thiserror;

/// Contains the common functionalities for defining errors..
#[macro_use]
pub mod common;
pub use self::common::*;

/// Contains traits and types for channels through which errors go.
pub mod emitter;

/// Contains the errors and warnings for the Kari lang.
pub mod error;
pub use self::errors::*;
