use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize)]
#[serde(rename = "sst")]
pub struct SharedStrings {
    #[serde(rename = "si")]
    pub si: Vec<Si>,

    #[serde(rename = "@xmlns")]
    xmlns: String,

    #[serde(rename = "@count")]
    count: String,

    #[serde(rename = "@uniqueCount")]
    unique_count: String,
}

#[derive(Serialize, Deserialize)]
pub struct Si {
    /// text
    #[serde(rename = "t")]
    pub t: String,

    #[serde(rename = "phoneticPr")]
    phonetic_pr: PhoneticPr,
}

#[derive(Serialize, Deserialize)]
pub struct PhoneticPr {
    #[serde(rename = "@fontId")]
    font_id: String,
}
