use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename = "workbook")]
pub struct Workbook {
    #[serde(rename = "sheets")]
    pub sheets: Sheets,

    #[serde(rename = "fileVersion")]
    file_version: FileVersion,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlternateContent {
    #[serde(rename = "Choice")]
    choice: Choice,

    #[serde(rename = "@xmlns:mc")]
    xmlns_mc: String,

    #[serde(rename = "@_prefix")]
    prefix: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Choice {
    #[serde(rename = "absPath")]
    abs_path: AbsPath,

    #[serde(rename = "@Requires")]
    requires: String,

    #[serde(rename = "@_prefix")]
    prefix: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AbsPath {
    #[serde(rename = "@xmlns:x15ac")]
    xmlns_x15_ac: String,

    #[serde(rename = "@url")]
    url: String,

    #[serde(rename = "@_prefix")]
    prefix: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BookViews {
    #[serde(rename = "workbookView")]
    workbook_view: WorkbookView,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkbookView {
    #[serde(rename = "@xWindow")]
    x_window: String,

    #[serde(rename = "@yWindow")]
    y_window: String,

    #[serde(rename = "@windowWidth")]
    window_width: String,

    #[serde(rename = "@windowHeight")]
    window_height: String,

    #[serde(rename = "@xr2:uid")]
    xr2_uid: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CalcPr {
    #[serde(rename = "@calcId")]
    calc_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtLst {
    #[serde(rename = "ext")]
    ext: Vec<Ext>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ext {
    #[serde(rename = "workbookPr")]
    workbook_pr: Option<ExtWorkbookPr>,

    #[serde(rename = "@xmlns:x15")]
    xmlns_x15: Option<String>,

    #[serde(rename = "@uri")]
    uri: String,

    #[serde(rename = "calcFeatures")]
    calc_features: Option<CalcFeatures>,

    #[serde(rename = "@xmlns:xcalcf")]
    xmlns_xcalcf: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CalcFeatures {
    #[serde(rename = "feature")]
    feature: Vec<Feature>,

    #[serde(rename = "@_prefix")]
    prefix: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Feature {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@_prefix")]
    prefix: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtWorkbookPr {
    #[serde(rename = "@chartTrackingRefBase")]
    chart_tracking_ref_base: String,

    #[serde(rename = "@_prefix")]
    prefix: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileVersion {
    #[serde(rename = "@appName")]
    app_name: String,

    #[serde(rename = "@lastEdited")]
    last_edited: String,

    #[serde(rename = "@lowestEdited")]
    lowest_edited: String,

    #[serde(rename = "@rupBuild")]
    rup_build: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RevisionPtr {
    #[serde(rename = "@revIDLastSave")]
    rev_id_last_save: String,

    #[serde(rename = "@documentId")]
    document_id: String,

    #[serde(rename = "@xr6:coauthVersionLast")]
    xr6_coauth_version_last: String,

    #[serde(rename = "@xr6:coauthVersionMax")]
    xr6_coauth_version_max: String,

    #[serde(rename = "@xr10:uidLastSave")]
    xr10_uid_last_save: String,

    #[serde(rename = "@_prefix")]
    prefix: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sheets {
    #[serde(rename = "sheet")]
    pub sheet: Vec<Sheet>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sheet {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@sheetId")]
    pub sheet_id: String,

    #[serde(rename = "@id")]
    id: String,
}
