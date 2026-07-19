//! Clean-Architecture-flavored Rust template. See the module docs on
//! [`core`], [`api`], and [`utils`] for how the layers fit together.

pub mod api;
pub mod config;
pub mod core;
pub mod error;
pub mod utils;

pub use error::{Error, Result};
