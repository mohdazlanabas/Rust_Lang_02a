pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red, Yellow, Blue,}

pub enum SecondaryColor {
    Orange, Green, Purple}
}

pub mod utils {
    use crate::kinds::*;
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange}
    }
