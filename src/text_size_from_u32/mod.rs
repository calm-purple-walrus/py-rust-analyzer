use pyo3::prelude::*;

use crate::types::TextSize;

#[pymethods]
impl TextSize {
    /// `impl From<u32> for TextSize`: `fn from(raw: u32) -> Self`
    #[staticmethod]
    #[pyo3(name = "from_u32")]
    fn from_u32(raw: u32) -> TextSize {
        TextSize { inner: ra_ap_syntax::TextSize::from(raw) }
    }
}
