use pyo3::prelude::*;

use crate::held::Held;
use crate::types::{AstBlockExpr, SyntaxToken};

#[pymethods]
impl AstBlockExpr {
    /// `pub fn async_token(&self) -> Option<SyntaxToken>`
    fn async_token(&self) -> Option<SyntaxToken> {
        self.inner.async_token().map(|inner| SyntaxToken { inner: Held::Own(inner) })
    }
}
