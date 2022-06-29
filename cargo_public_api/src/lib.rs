//! # Art
//! 
//! An art library for colors and art

// This library may be hard for end-users to figure out how to use 
// since they have to find out that PrimaryColor is in a kinds module 
// and mix is in utils, to make this library simpler to end-users
// we can use `pub use` to export items to the top level
pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors acording to RYB model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to RYB model
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts
    pub fn mix(color1: PrimaryColor, color2: PrimaryColor) -> SecondaryColor {

    }
}