use crate::types::{AstBinExpr, AstExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstBinExpr {
    /// `fn rhs(&self) -> Option<Expr>`
    fn rhs(&self) -> Option<AstExpr> {
        self.inner.rhs().map(|inner| AstExpr { inner })
    }
}
