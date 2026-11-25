use crate::types::{AstExpr, AstMatchArm};
use pyo3::prelude::*;

#[pymethods]
impl AstMatchArm {
    /// `fn expr(&self) -> Option<Expr>`
    fn expr(&self) -> Option<AstExpr> {
        self.inner.expr().map(|inner| AstExpr { inner })
    }
}
