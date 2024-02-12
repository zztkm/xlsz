use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SharedStrings {
    #[serde(rename = "sst")]
    sst: Sst,
}

#[derive(Serialize, Deserialize)]
pub struct Sst {
    #[serde(rename = "si")]
    si: Vec<Si>,

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
    t: String,

    #[serde(rename = "phoneticPr")]
    phonetic_pr: PhoneticPr,
}

#[derive(Serialize, Deserialize)]
pub struct PhoneticPr {
    #[serde(rename = "@fontId")]
    font_id: String,
}
