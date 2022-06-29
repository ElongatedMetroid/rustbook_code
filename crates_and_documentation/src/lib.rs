// The //! addes documentation to the item that contains the comments
// These comments are found in the lib.rs file which is the crate root, meaning they are documenting the entire crate
//! This crate is for learning how to document code in Rust/Cargo

// Documentation comments start with 3 slashes and are formated in markdown
// You can run the examples in documentation with cargo test
/// Adds one to x
/// # Examples
/// 
/// ```
/// let five = 5;
/// 
/// assert_eq!(five, crates_and_documentation::add_one(4));
/// ```
/// 
/// # Panics
/// 
/// # Errors
/// 
/// # Safety
pub fn add_one(x: i32) -> i32 {
    x + 1
}