use pyo3::prelude::*;

use crate::types::{SyntaxNode, TextRange};

#[pymethods]
impl SyntaxNode {
    /// `pub fn text_range(&self) -> TextRange`
    fn text_range(&self) -> TextRange {
        TextRange { inner: self.inner.text_range() }
    }
}
