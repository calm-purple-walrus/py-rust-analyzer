use crate::iter::Iter;
use crate::types::{AstArgList, AstExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstArgList {
    /// `fn args(&self) -> AstChildren<Expr>`
    fn args(&self) -> Iter {
        Iter::new(
            self.inner.args(),
            |py, inner| Ok(Py::new(py, AstExpr { inner })?.into_any()),
            None,
        )
    }
}
