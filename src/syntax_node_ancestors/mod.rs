use pyo3::prelude::*;

use crate::held::Held;
use crate::iter::Iter;
use crate::types::SyntaxNode;

#[pymethods]
impl SyntaxNode {
    /// `pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode>`
    fn ancestors(&self) -> Iter {
        Iter::new(
            self.inner.ancestors(),
            |py, inner| Ok(Py::new(py, SyntaxNode { inner: Held::Own(inner) })?.into_any()),
            None,
        )
    }
}
