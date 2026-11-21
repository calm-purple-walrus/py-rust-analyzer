use pyo3::prelude::*;

use crate::types::{AstFieldExpr, AstExpr};

#[pymethods]
impl AstFieldExpr {
    /// `pub fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
