mod shared_strings;
mod styles;
mod workbook;
mod worksheets;

pub use shared_strings::SharedStrings;
pub use styles::StyleSheet;
pub use workbook::Workbook;
pub use worksheets::Worksheet;

pub struct Xl {
    pub worksheets: Vec<Worksheet>,
    pub shared_strings: SharedStrings,
    pub stylesheet: StyleSheet,
    pub workbook: Workbook,
}
