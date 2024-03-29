//! Art
//! 
//! This crate provides a library for modeling artistic concepts.

// To re-export the kinds module, we add pub use self::kinds::PrimaryColor; and pub use self::kinds::SecondaryColor; to the top of the lib.rs file.
// This makes it so that users of the crate can access the PrimaryColor and SecondaryColor types directly from the crate root.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
  /// The primary colors according to the RYB color model.
  pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
  }

  /// The secondary colors according to the RYB color model.
  pub enum SecondaryColor {
    Orange,
    Green,
    Purple,
  }
}

pub mod utils {
  use crate::kinds::*;

  /// Combines two primary colors in equal amounts to create
  /// a secondary color.
  pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
      // --snip--
      SecondaryColor::Orange
  }
}