use crate::types::{AstExpr, AstMethodCallExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstMethodCallExpr {
    /// `fn receiver(&self) -> Option<Expr>`
    fn receiver(&self) -> Option<AstExpr> {
        self.inner.receiver().map(|inner| AstExpr { inner })
    }
}
