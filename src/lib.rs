use pyo3::prelude::*;

mod xlsx;

/// A Python module implemented in Rust.
#[pymodule]
fn _xlsx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<xlsx::Xlsx>()?;
    Ok(())
}
