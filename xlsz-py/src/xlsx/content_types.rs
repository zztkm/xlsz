use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize)]
#[serde(rename = "Types")]
pub struct ContentTypes {
    #[serde(rename = "Default")]
    pub types_default: Vec<Default>,

    #[serde(rename = "Override")]
    pub types_override: Vec<Override>,

    #[serde(rename = "@xmlns")]
    pub xmlns: String,
}

#[derive(Serialize, Deserialize)]
pub struct Default {
    #[serde(rename = "@Extension")]
    extension: String,

    #[serde(rename = "@ContentType")]
    content_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Override {
    #[serde(rename = "@PartName")]
    part_name: String,

    #[serde(rename = "@ContentType")]
    content_type: String,
}
