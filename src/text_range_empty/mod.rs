use pyo3::prelude::*;

use crate::types::{TextRange, TextSize};

#[pymethods]
impl TextRange {
    /// `pub const fn empty(offset: TextSize) -> TextRange`
    #[staticmethod]
    fn empty(offset: &TextSize) -> TextRange {
        TextRange { inner: ra_ap_syntax::TextRange::empty(offset.inner) }
    }
}
