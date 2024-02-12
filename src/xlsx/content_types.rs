use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ContentTypes {
    #[serde(rename = "Types")]
    types: Types,
}

#[derive(Serialize, Deserialize)]
pub struct Types {
    #[serde(rename = "Default")]
    types_default: Vec<Default>,

    #[serde(rename = "Override")]
    types_override: Vec<Override>,

    #[serde(rename = "@xmlns")]
    xmlns: String,
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
