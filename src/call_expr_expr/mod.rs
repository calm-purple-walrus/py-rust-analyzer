use pyo3::prelude::*;

use crate::types::{AstCallExpr, AstExpr};

#[pymethods]
impl AstCallExpr {
    /// `pub fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
