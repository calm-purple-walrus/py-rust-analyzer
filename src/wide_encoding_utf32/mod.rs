use pyo3::prelude::*;

use crate::types::WideEncoding;

#[pymethods]
impl WideEncoding {
    /// The variant `WideEncoding::Utf32`, made.
    #[staticmethod]
    #[pyo3(name = "Utf32")]
    fn utf32() -> WideEncoding {
        WideEncoding { inner: ra_ap_ide_db::line_index::WideEncoding::Utf32 }
    }
}
