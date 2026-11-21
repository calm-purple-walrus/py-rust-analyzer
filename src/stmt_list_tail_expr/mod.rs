use pyo3::prelude::*;

use crate::types::{AstStmtList, AstExpr};

#[pymethods]
impl AstStmtList {
    /// `pub fn tail_expr(&self) -> Option<Expr>`
    fn tail_expr(&self) -> Option<AstExpr> {
        self.inner.tail_expr().map(|inner| AstExpr { inner })
    }
}
