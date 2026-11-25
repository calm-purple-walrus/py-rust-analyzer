use crate::types::{AstExpr, AstMatchGuard};
use pyo3::prelude::*;

#[pymethods]
impl AstMatchGuard {
    /// `fn condition(&self) -> Option<Expr>`
    fn condition(&self) -> Option<AstExpr> {
        self.inner.condition().map(|inner| AstExpr { inner })
    }
}
