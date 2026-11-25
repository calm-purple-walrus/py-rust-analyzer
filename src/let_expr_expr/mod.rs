use pyo3::prelude::*;

use crate::types::{AstExpr, AstLetExpr};

#[pymethods]
impl AstLetExpr {
    /// `pub fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
