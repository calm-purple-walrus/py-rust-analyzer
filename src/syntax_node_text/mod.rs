use pyo3::prelude::*;

use crate::types::{SyntaxNode, SyntaxText};

#[pymethods]
impl SyntaxNode {
    /// `pub fn text(&self) -> SyntaxText`
    fn text(&self) -> SyntaxText {
        SyntaxText { inner: self.inner.text() }
    }
}
