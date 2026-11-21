use pyo3::prelude::*;

use crate::held::Held;
use crate::types::SyntaxNode;

#[pymethods]
impl SyntaxNode {
    /// `impl Clone for SyntaxNode`
    fn clone(&self) -> SyntaxNode {
        SyntaxNode { inner: Held::Own((*self.inner).clone()) }
    }
}
