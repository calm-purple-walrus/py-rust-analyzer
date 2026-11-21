use pyo3::prelude::*;

use crate::types::{SyntaxNode, SyntaxToken};

#[pymethods]
impl SyntaxNode {
    /// `pub fn first_token(&self) -> Option<SyntaxToken>`
    fn first_token(&self) -> Option<SyntaxToken> {
        self.inner.first_token().map(|inner| SyntaxToken { inner })
    }
}
