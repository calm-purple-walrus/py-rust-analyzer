use pyo3::prelude::*;

use crate::held::Held;
use crate::types::{SyntaxElement, SyntaxNode};

#[pymethods]
impl SyntaxElement {
    /// `pub fn into_node(self) -> Option<N>`, with `N` the `SyntaxNode`. It takes the value, so it is given a
    /// copy.
    fn into_node(&self) -> Option<SyntaxNode> {
        self.inner.clone().into_node().map(|inner| SyntaxNode { inner: Held::Own(inner) })
    }
}
