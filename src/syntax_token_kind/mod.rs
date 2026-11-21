use pyo3::prelude::*;

use crate::types::{SyntaxKind, SyntaxToken};

#[pymethods]
impl SyntaxToken {
    /// `pub fn kind(&self) -> SyntaxKind`
    fn kind(&self) -> SyntaxKind {
        SyntaxKind { inner: self.inner.kind() }
    }
}
