use crate::types::{AstExpr, AstForExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstForExpr {
    /// `fn iterable(&self) -> Option<Expr>`
    fn iterable(&self) -> Option<AstExpr> {
        self.inner.iterable().map(|inner| AstExpr { inner })
    }
}
