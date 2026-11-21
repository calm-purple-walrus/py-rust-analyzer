use pyo3::prelude::*;

use crate::types::{AstParenExpr, AstExpr};

#[pymethods]
impl AstParenExpr {
    /// `pub fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
