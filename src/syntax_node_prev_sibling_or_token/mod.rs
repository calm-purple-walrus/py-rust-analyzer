use pyo3::prelude::*;

use crate::types::{SyntaxElement, SyntaxNode};

#[pymethods]
impl SyntaxNode {
    /// `pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement>`
    fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
        self.inner.prev_sibling_or_token().map(|inner| SyntaxElement { inner })
    }
}
