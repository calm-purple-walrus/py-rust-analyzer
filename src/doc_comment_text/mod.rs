use pyo3::prelude::*;

use crate::types::AstDocComment;

#[pymethods]
impl AstDocComment {
    /// `fn text(&self) -> &str`
    fn text(&self) -> &str {
        self.inner.text()
    }
}
