use pyo3::prelude::*;

use crate::types::{AstExprStmt, AstExpr};

#[pymethods]
impl AstExprStmt {
    /// `pub fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
