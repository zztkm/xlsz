mod shared_strings;
mod styles;
mod workbook;
mod worksheet;

use pyo3::prelude::*;
pub use shared_strings::SharedStrings;
pub use styles::StyleSheet;
pub use workbook::Workbook;
pub use worksheet::{Worksheet, C};

#[pyclass]
#[derive(Clone)]
pub struct Xl {
    pub stylesheet: StyleSheet,
    pub workbook: Workbook,
}

impl Xl {
    pub fn new(stylesheet: StyleSheet, workbook: Workbook) -> Self {
        // TODO: worksheets はデータがでかいのでシートデータを取得するときのみデータを保持するようにする
        Xl {
            stylesheet,
            workbook,
        }
    }

    pub fn get_sheet_names(&self) -> Vec<String> {
        self.workbook
            .sheets
            .sheet
            .iter()
            .map(|sheet| sheet.name.clone())
            .collect()
    }

    pub fn get_sheet_id(&self, sheet_name: String) -> String {
        self.workbook
            .sheets
            .sheet
            .iter()
            .find(|sheet| sheet.name == sheet_name)
            .unwrap()
            .sheet_id
            .clone()
    }
}
