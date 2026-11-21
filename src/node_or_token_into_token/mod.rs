use pyo3::prelude::*;

use crate::held::Held;
use crate::types::{SyntaxElement, SyntaxToken};

#[pymethods]
impl SyntaxElement {
    /// `pub fn into_token(self) -> Option<SyntaxToken>`. It takes the value, so it is given a copy.
    fn into_token(&self) -> Option<SyntaxToken> {
        self.inner.clone().into_token().map(|inner| SyntaxToken { inner: Held::Own(inner) })
    }
}
