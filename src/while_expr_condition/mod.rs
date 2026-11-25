use crate::types::{AstExpr, AstWhileExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstWhileExpr {
    /// `fn condition(&self) -> Option<Expr>`
    fn condition(&self) -> Option<AstExpr> {
        self.inner.condition().map(|inner| AstExpr { inner })
    }
}
