use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{SyntaxElement, SyntaxNode};

#[pymethods]
impl SyntaxNode {
    /// `pub fn descendants_with_tokens(&self) -> impl Iterator<Item = SyntaxElement>`
    fn descendants_with_tokens(&self) -> Iter {
        Iter::new(self.inner.descendants_with_tokens(), |py, inner| Ok(Py::new(py, SyntaxElement { inner })?.into_any()), None)
    }
}
