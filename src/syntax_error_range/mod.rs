use pyo3::prelude::*;

use crate::types::{SyntaxError, TextRange};

#[pymethods]
impl SyntaxError {
    /// `pub fn range(&self) -> TextRange`
    fn range(&self) -> TextRange {
        TextRange { inner: self.inner.range() }
    }
}
