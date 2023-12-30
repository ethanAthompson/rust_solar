#![crate_name = "rust_solar"]
//! # Rust solar
//!
//! A library for cosmic calendar.

pub fn main() {}



#[doc = "Returns a String"]
#[doc(alias = "Stringly")]
pub fn stringly() -> String {
    String::new()
}

#[doc = "Types for calculating a sol"]
pub mod mars_types {
    pub enum Dates {
        DSOL,
        MSOL,
        KATA
    }
}
