use pyo3::prelude::*;
use quick_xml::de::from_str;
use std::{fs, io::Read};

mod content_types;
mod xl;

use content_types::ContentTypes;
use xl::{SharedStrings, StyleSheet, Workbook, Worksheet, Xl};

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
        println!("archive");
        let content_types: ContentTypes = read_file(&mut archive, "[Content_Types].xml");
        println!("content_types");
        let shared_strings: SharedStrings = read_file(&mut archive, "xl/sharedStrings.xml");
        println!("shared_strings");
        let stylesheet: StyleSheet = read_file(&mut archive, "xl/styles.xml");
        println!("styles");
        let workbook: Workbook = read_file(&mut archive, "xl/workbook.xml");
        println!("workbook");

        let worksheets: Vec<Worksheet> = workbook
            .sheets
            .sheet
            .iter()
            .map(|sheet| {
                let filename = format!("xl/worksheets/sheet{}.xml", sheet.sheet_id);
                let sheet: Worksheet = read_file(&mut archive, &filename);
                sheet
            })
            .collect();

        Xlsx {
            content_types,
            xl: Xl {
                worksheets,
                shared_strings,
                stylesheet,
                workbook,
            },
            archive,
        }
    }

    fn get_sheet_names(&self) -> Vec<String> {
        self.xl
            .workbook
            .sheets
            .sheet
            .iter()
            .map(|sheet| sheet.name.clone())
            .collect()
    }
}
