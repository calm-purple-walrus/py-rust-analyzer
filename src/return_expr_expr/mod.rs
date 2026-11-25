use crate::types::{AstExpr, AstReturnExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstReturnExpr {
    /// `fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
