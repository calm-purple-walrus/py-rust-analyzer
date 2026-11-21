use pyo3::prelude::*;

use crate::types::{SyntaxToken, TextRange};

#[pymethods]
impl SyntaxToken {
    /// `pub fn text_range(&self) -> TextRange`
    fn text_range(&self) -> TextRange {
        TextRange { inner: self.inner.text_range() }
    }
}
