use pyo3::prelude::*;
use quick_xml::de::from_str;
use std::{io::Read, path::PathBuf};

mod content_types;
mod xl;

use content_types::ContentTypes;
use xl::{SharedStrings, Sheet, Styles, Xl};

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
    pub fn new(xlsx_file: PathBuf) -> Self {
        let zipfile = std::fs::File::open(xlsx_file).unwrap();

        let mut archive = zip::ZipArchive::new(zipfile).unwrap();

        // ファイルを読み込む
        let mut archive = zip::ZipArchive::new(std::fs::File::open(xlsx_file).unwrap()).unwrap();
        let content_types: ContentTypes = read_file(&mut archive, "[Content_Types].xml");

        Xlsx {
            content_types: ContentTypes::new(&mut archive).unwrap(),
            xl: Xl::new(&mut archive).unwrap(),
            archive,
        }
    }
}
