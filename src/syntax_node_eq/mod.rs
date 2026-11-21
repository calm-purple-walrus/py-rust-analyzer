use pyo3::prelude::*;

use crate::types::SyntaxNode;

#[pymethods]
impl SyntaxNode {
    /// `impl PartialEq for SyntaxNode`
    fn __eq__(&self, other: &SyntaxNode) -> bool {
        *self.inner == *other.inner
    }
}
