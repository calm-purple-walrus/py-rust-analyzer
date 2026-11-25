use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{SyntaxElement, SyntaxNode};

#[pymethods]
impl SyntaxNode {
    /// `pub fn children_with_tokens(&self) -> SyntaxElementChildren`
    fn children_with_tokens(&self) -> Iter {
        Iter::new(
            self.inner.children_with_tokens(),
            |py, inner| Ok(Py::new(py, SyntaxElement { inner })?.into_any()),
            None,
        )
    }
}
