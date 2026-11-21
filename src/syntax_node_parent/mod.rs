use pyo3::prelude::*;

use crate::held::Held;
use crate::types::SyntaxNode;

#[pymethods]
impl SyntaxNode {
    /// `pub fn parent(&self) -> Option<SyntaxNode>`
    fn parent(&self) -> Option<SyntaxNode> {
        self.inner.parent().map(|inner| SyntaxNode { inner: Held::Own(inner) })
    }
}
