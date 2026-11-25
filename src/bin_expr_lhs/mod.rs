use crate::types::{AstBinExpr, AstExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstBinExpr {
    /// `fn lhs(&self) -> Option<Expr>`
    fn lhs(&self) -> Option<AstExpr> {
        self.inner.lhs().map(|inner| AstExpr { inner })
    }
}
