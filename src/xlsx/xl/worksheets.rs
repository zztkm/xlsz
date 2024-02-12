use serde::{Deserialize, Serialize};

/// シート毎のデータを表す構造体
#[derive(Serialize, Deserialize)]
pub struct Sheet {
    #[serde(rename = "worksheet")]
    worksheet: Worksheet,
}

#[derive(Serialize, Deserialize)]
pub struct Worksheet {
    #[serde(rename = "dimension")]
    dimension: Dimension,

    #[serde(rename = "sheetViews")]
    sheet_views: SheetViews,

    #[serde(rename = "sheetFormatPr")]
    sheet_format_pr: SheetFormatPr,

    #[serde(rename = "sheetData")]
    sheet_data: SheetData,

    #[serde(rename = "phoneticPr")]
    phonetic_pr: PhoneticPr,

    #[serde(rename = "pageMargins")]
    page_margins: PageMargins,

    #[serde(rename = "@xmlns")]
    xmlns: String,

    #[serde(rename = "@xmlns:r")]
    xmlns_r: String,

    #[serde(rename = "@xmlns:mc")]
    xmlns_mc: String,

    #[serde(rename = "@xmlns:x14ac")]
    xmlns_x14_ac: String,

    #[serde(rename = "@xmlns:xr")]
    xmlns_xr: String,

    #[serde(rename = "@xmlns:xr2")]
    xmlns_xr2: String,

    #[serde(rename = "@xmlns:xr3")]
    xmlns_xr3: String,

    #[serde(rename = "@mc:Ignorable")]
    mc_ignorable: String,

    #[serde(rename = "@xr:uid")]
    xr_uid: String,
}

#[derive(Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "@ref")]
    dimension_ref: String,
}

#[derive(Serialize, Deserialize)]
pub struct PageMargins {
    #[serde(rename = "@left")]
    left: String,

    #[serde(rename = "@right")]
    right: String,

    #[serde(rename = "@top")]
    top: String,

    #[serde(rename = "@bottom")]
    bottom: String,

    #[serde(rename = "@header")]
    header: String,

    #[serde(rename = "@footer")]
    footer: String,
}

#[derive(Serialize, Deserialize)]
pub struct PhoneticPr {
    #[serde(rename = "@fontId")]
    font_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SheetData {
    #[serde(rename = "row")]
    row: Vec<Row>,
}

#[derive(Serialize, Deserialize)]
pub struct Row {
    #[serde(rename = "c")]
    c: C,

    #[serde(rename = "@r")]
    r: String,

    #[serde(rename = "@spans")]
    spans: Spans,

    #[serde(rename = "@x14ac:dyDescent")]
    x14_ac_dy_descent: String,
}

/// セル情報を表す構造体
#[derive(Serialize, Deserialize)]
pub struct C {
    /// value
    #[serde(rename = "v")]
    v: Option<String>,

    #[serde(rename = "@r")]
    r: String,

    /// if t = "s" then v is index of shared string.
    #[serde(rename = "@t")]
    t: Option<String>,

    /// if s is not None then s is index of style(cellXfs).
    #[serde(rename = "@s")]
    s: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SheetFormatPr {
    #[serde(rename = "@defaultRowHeight")]
    default_row_height: String,

    #[serde(rename = "@x14ac:dyDescent")]
    x14_ac_dy_descent: String,
}

#[derive(Serialize, Deserialize)]
pub struct SheetViews {
    #[serde(rename = "sheetView")]
    sheet_view: SheetView,
}

#[derive(Serialize, Deserialize)]
pub struct SheetView {
    #[serde(rename = "@tabSelected")]
    tab_selected: String,

    #[serde(rename = "@workbookViewId")]
    workbook_view_id: String,
}

#[derive(Serialize, Deserialize)]
pub enum Spans {
    #[serde(rename = "1:1")]
    The11,
}
