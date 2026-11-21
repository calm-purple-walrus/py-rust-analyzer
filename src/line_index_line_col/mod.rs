use pyo3::prelude::*;

use crate::types::{LineCol, LineIndex, TextSize};

#[pymethods]
impl LineIndex {
    /// `pub fn line_col(&self, offset: TextSize) -> LineCol`
    fn line_col(&self, offset: &TextSize) -> LineCol {
        LineCol { inner: self.inner.line_col(offset.inner) }
    }
}
