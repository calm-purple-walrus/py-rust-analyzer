use pyo3::prelude::*;

use crate::types::TextRange;

#[pymethods]
impl TextRange {
    /// `impl Debug for TextRange`
    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}
