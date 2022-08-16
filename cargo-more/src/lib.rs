//! # Art
//! 
//! A library for modeling artistic concepts
//! 

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

/// The primary colors according ot the RYB color model
pub mod kinds {
    pub enum PrimaryColor {
        Red, Yellow, Blue
    }

    pub enum SecondaryColor {
        Orange, Green, Purple
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(x1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
