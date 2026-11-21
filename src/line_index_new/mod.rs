use pyo3::prelude::*;

use crate::types::LineIndex;

#[pymethods]
impl LineIndex {
    /// `pub fn new(text: &str) -> LineIndex`
    #[staticmethod]
    fn new(text: &str) -> LineIndex {
        LineIndex { inner: ra_ap_ide_db::line_index::LineIndex::new(text) }
    }
}
