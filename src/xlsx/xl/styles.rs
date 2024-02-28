use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize)]
#[serde(rename = "styleSheet")]
pub struct StyleSheet {
    #[serde(rename = "numFmts")]
    num_fmts: NumFmts,

    #[serde(rename = "cellXfs")]
    cell_xfs: CellXfs,

    #[serde(rename = "fonts")]
    fonts: Fonts,

    #[serde(rename = "fills")]
    fills: Fills,

    #[serde(rename = "borders")]
    borders: Borders,

    #[serde(rename = "cellStyleXfs")]
    cell_style_xfs: CellStyleXfs,

    #[serde(rename = "cellStyles")]
    cell_styles: CellStyles,

    #[serde(rename = "dxfs")]
    dxfs: Dxfs,

    #[serde(rename = "tableStyles")]
    table_styles: TableStyles,

    #[serde(rename = "@xmlns")]
    xmlns: String,

    #[serde(rename = "@xmlns:mc")]
    xmlns_mc: String,

    #[serde(rename = "@xmlns:x14ac")]
    xmlns_x14_ac: String,

    #[serde(rename = "@xmlns:x16r2")]
    xmlns_x16_r2: String,

    #[serde(rename = "@xmlns:xr")]
    xmlns_xr: String,
}

#[derive(Serialize, Deserialize)]
pub struct Borders {
    #[serde(rename = "border")]
    border: Border,

    #[serde(rename = "@count")]
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct Border {
    #[serde(rename = "left")]
    left: String,

    #[serde(rename = "right")]
    right: String,

    #[serde(rename = "top")]
    top: String,

    #[serde(rename = "bottom")]
    bottom: String,

    #[serde(rename = "diagonal")]
    diagonal: String,
}

#[derive(Serialize, Deserialize)]
pub struct CellStyleXfs {
    #[serde(rename = "xf")]
    xf: CellStyleXfsXf,

    #[serde(rename = "@count")]
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct CellStyleXfsXf {
    #[serde(rename = "alignment")]
    alignment: Alignment,

    #[serde(rename = "@numFmtId")]
    num_fmt_id: String,

    #[serde(rename = "@fontId")]
    font_id: String,

    #[serde(rename = "@fillId")]
    fill_id: String,

    #[serde(rename = "@borderId")]
    border_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Alignment {
    #[serde(rename = "@vertical")]
    vertical: String,
}

#[derive(Serialize, Deserialize)]
pub struct CellStyles {
    #[serde(rename = "cellStyle")]
    cell_style: CellStyle,

    #[serde(rename = "@count")]
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct CellStyle {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@xfId")]
    xf_id: String,

    #[serde(rename = "@builtinId")]
    builtin_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CellXfs {
    #[serde(rename = "xf")]
    xf: Vec<XfElement>,

    #[serde(rename = "@count")]
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct XfElement {
    #[serde(rename = "alignment")]
    alignment: Alignment,

    #[serde(rename = "@numFmtId")]
    num_fmt_id: String,

    #[serde(rename = "@fontId")]
    font_id: String,

    #[serde(rename = "@fillId")]
    fill_id: String,

    #[serde(rename = "@borderId")]
    border_id: String,

    #[serde(rename = "@xfId")]
    xf_id: String,

    #[serde(rename = "@applyNumberFormat")]
    apply_number_format: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Dxfs {
    #[serde(rename = "@count")]
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct Fills {
    #[serde(rename = "fill")]
    fill: Vec<Fill>,

    #[serde(rename = "@count")]
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct Fill {
    #[serde(rename = "patternFill")]
    pattern_fill: PatternFill,
}

#[derive(Serialize, Deserialize)]
pub struct PatternFill {
    #[serde(rename = "@patternType")]
    pattern_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Fonts {
    #[serde(rename = "font")]
    font: Vec<Font>,

    #[serde(rename = "@count")]
    count: String,

    #[serde(rename = "@x14ac:knownFonts")]
    #[serde(default)]
    x14_ac_known_fonts: String,
}

#[derive(Serialize, Deserialize)]
pub struct Font {
    #[serde(rename = "sz")]
    sz: Charset,

    #[serde(rename = "color")]
    color: Option<Color>,

    #[serde(rename = "name")]
    name: Charset,

    #[serde(rename = "family")]
    family: Charset,

    #[serde(rename = "charset")]
    charset: Charset,

    #[serde(rename = "scheme")]
    scheme: Charset,
}

#[derive(Serialize, Deserialize)]
pub struct Charset {
    #[serde(rename = "@val")]
    val: String,
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    #[serde(rename = "@theme")]
    theme: String,
}

#[derive(Serialize, Deserialize)]
pub struct NumFmts {
    #[serde(rename = "numFmt")]
    num_fmt: Vec<NumFmt>,

    #[serde(rename = "@count")]
    count: String,
}

#[derive(Serialize, Deserialize)]
pub struct NumFmt {
    #[serde(rename = "@numFmtId")]
    num_fmt_id: String,

    #[serde(rename = "@formatCode")]
    format_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct TableStyles {
    #[serde(rename = "@count")]
    count: String,

    #[serde(rename = "@defaultTableStyle")]
    default_table_style: String,

    #[serde(rename = "@defaultPivotStyle")]
    default_pivot_style: String,
}
