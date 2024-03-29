// RENAME TO lib.rs TO MAKE IT A LIBRARY CRATE
// Documentation Comments - start with /// or //! (for the whole module) and use markdown syntax
// Rust will run the doc comments as a test and check that the code compiles
// Why is this file named lib.rs? Because it is a library crate. If it was a binary crate, it would be named main.rs
// Why can't we use any name? Because Cargo expects the library crate to be named lib.rs and the binary crate to be named main.rs

// To build and open HTML documentation for the crate:
// cargo doc --open

// Commonly used sections in documentation comments:
// # Examples
// # Panics
// # Errors
// # Safety 
// etc.

//! # My Crates
//! 
//! `my_crates` is a collection of utilities to make performing certain calculations more convenient.
//! calculations more convenient. inner doc comments like this (starting with `//!` or `/*!`) can only appear before items

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}