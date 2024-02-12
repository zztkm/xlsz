mod shared_strings;
mod styles;
mod worksheets;

pub use shared_strings::SharedStrings;
pub use styles::Styles;
pub use worksheets::Sheet;

pub struct Xl {
    pub worksheets: Vec<Sheet>,
    pub shared_strings: SharedStrings,
    pub styles: Styles,
}
