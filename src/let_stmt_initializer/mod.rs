use pyo3::prelude::*;

use crate::types::{AstExpr, AstLetStmt};

#[pymethods]
impl AstLetStmt {
    /// `pub fn initializer(&self) -> Option<Expr>`
    fn initializer(&self) -> Option<AstExpr> {
        self.inner.initializer().map(|inner| AstExpr { inner })
    }
}
