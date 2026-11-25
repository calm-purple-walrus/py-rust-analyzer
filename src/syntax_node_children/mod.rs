use pyo3::prelude::*;

use crate::held::Held;
use crate::iter::Iter;
use crate::types::SyntaxNode;

#[pymethods]
impl SyntaxNode {
    /// `pub fn children(&self) -> SyntaxNodeChildren`
    fn children(&self) -> Iter {
        Iter::new(
            self.inner.children(),
            |py, inner| Ok(Py::new(py, SyntaxNode { inner: Held::Own(inner) })?.into_any()),
            None,
        )
    }
}
