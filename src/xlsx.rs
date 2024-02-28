use pyo3::iter::IterNextOutput;
use pyo3::prelude::*;
use quick_xml::de::from_str;
use std::{fs, io::Read};

mod content_types;
mod xl;

use content_types::ContentTypes;
use xl::{SharedStrings, StyleSheet, Workbook, Worksheet, Xl, C};

#[pyclass]
pub struct SheetValuesIter {
    sheet: Worksheet,
    shared_strings: SharedStrings,
    row_index: usize,
    length: usize,
}

#[pymethods]
impl SheetValuesIter {
    #[new]
    pub fn new(sheetdata: &str, sstdata: &str) -> Self {
        let sheet: Worksheet = from_str(sheetdata).unwrap();
        let shared_strings: SharedStrings = from_str(sstdata).unwrap();
        let length = sheet.sheet_data.row.len();
        SheetValuesIter {
            sheet,
            shared_strings,
            row_index: 0,
            length,
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> IterNextOutput<Vec<String>, usize> {
        if self.row_index < self.length {
            let row = &self.sheet.sheet_data.row[self.row_index];
            self.row_index += 1;
            let rowdata = row.c.iter().map(|cell| self.get_cell_value(cell)).collect();
            IterNextOutput::Yield(rowdata)
        } else {
            IterNextOutput::Return(self.length)
        }
    }

    fn get_cell_value(&self, cell: &C) -> String {
        match &cell.t {
            Some(t) => {
                if t == "s" {
                    let index = cell.v.parse::<usize>().unwrap();
                    self.shared_strings.si[index].t.clone()
                } else {
                    cell.v.clone()
                }
            }
            None => cell.v.clone(),
        }
    }
}

/// Xlsx ファイルの構造を表す構造体
#[pyclass]
pub struct Xlsx {
    pub content_types: ContentTypes,
    pub xl: Xl,

    /// private field
    archive: zip::ZipArchive<std::fs::File>,
}

fn read_file<T: serde::de::DeserializeOwned>(
    archive: &mut zip::ZipArchive<std::fs::File>,
    filename: &str,
) -> T {
    let mut file = archive.by_name(filename).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    from_str(&buf).unwrap()
}

#[pymethods]
impl Xlsx {
    /// Xlsx ファイルの構造を表す構造体を作成する
    /// shared_strings と worksheet は get_sheet_values が呼ばれたタイミングで読み込む
    /// TODO: error handling: https://pyo3.rs/v0.20.2/function/error_handling
    #[new]
    pub fn new(xlsx_file: String) -> Self {
        // カレントディレクトリを取得
        let current_dir = std::env::current_dir().unwrap();
        println!("{:?}", current_dir);
        // ファイルを読み込む
        println!("{:?}", xlsx_file);
        let fname = std::path::Path::new(&xlsx_file);
        let display = fname.display();
        let file = match fs::File::open(fname) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let content_types: ContentTypes = read_file(&mut archive, "[Content_Types].xml");
        let stylesheet: StyleSheet = read_file(&mut archive, "xl/styles.xml");
        let workbook: Workbook = read_file(&mut archive, "xl/workbook.xml");

        let xl = Xl::new(stylesheet, workbook);

        Xlsx {
            content_types,
            xl,
            archive,
        }
    }

    pub fn get_sheet_names(&self) -> Vec<String> {
        self.xl.get_sheet_names()
    }

    pub fn get_sheet_values(&mut self, sheet_name: String) -> SheetValuesIter {
        let sheet_id = self.xl.get_sheet_id(sheet_name);
        let filename = format!("xl/worksheets/sheet{}.xml", sheet_id);
        let data = self.read_file(&filename);
        let sstdata = self.read_file("xl/sharedStrings.xml");
        SheetValuesIter::new(&data, &sstdata)
    }

    fn read_file(&mut self, filename: &str) -> String {
        let mut file = self.archive.by_name(filename).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        buf
    }
}
