use pyo3::prelude::*;

use crate::types::{SyntaxElement, SyntaxToken};

#[pymethods]
impl SyntaxToken {
    /// `pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement>`
    fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.inner.prev_sibling_or_token().map(|inner| SyntaxElement { inner })
    }
}
