use crate::iter::Iter;
use crate::types::{AstArrayExprKind, AstExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstArrayExprKind {
    /// The `ArrayExprKind::ElementList` variant, read.
    #[pyo3(name = "as_ElementList")]
    fn as_element_list(&self) -> Option<Iter> {
        match &self.inner {
            ra_ap_syntax::ast::ArrayExprKind::ElementList(items) => Some(Iter::new(
                items.clone(),
                |py, inner| Ok(Py::new(py, AstExpr { inner })?.into_any()),
                None,
            )),
            _ => None,
        }
    }
}
