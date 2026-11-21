use pyo3::prelude::*;

use crate::iter::Iter;
use crate::types::{AstStmt, AstStmtList};

#[pymethods]
impl AstStmtList {
    /// `pub fn statements(&self) -> AstChildren<Stmt>`
    fn statements(&self) -> Iter {
        Iter::new(self.inner.statements(), |py, inner| Ok(Py::new(py, AstStmt { inner })?.into_any()), None)
    }
}
