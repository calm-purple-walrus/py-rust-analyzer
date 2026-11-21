use pyo3::prelude::*;

use crate::types::{SyntaxKind, SyntaxNode};

#[pymethods]
impl SyntaxNode {
    /// `pub fn kind(&self) -> SyntaxKind`
    fn kind(&self) -> SyntaxKind {
        SyntaxKind { inner: self.inner.kind() }
    }
}
