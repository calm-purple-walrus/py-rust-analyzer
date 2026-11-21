use pyo3::prelude::*;
use ra_ap_syntax::ast::HasAttrs;

use crate::iter::Iter;
use crate::types::{AstAttr, AstFn};

#[pymethods]
impl AstFn {
    /// `HasAttrs::attrs`: `fn attrs(&self) -> AstChildren<ast::Attr>`
    fn attrs(&self) -> Iter {
        Iter::new(self.inner.attrs(), |py, inner| Ok(Py::new(py, AstAttr { inner })?.into_any()), None)
    }
}
