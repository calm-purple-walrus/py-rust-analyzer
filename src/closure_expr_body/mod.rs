use crate::types::{AstClosureExpr, AstExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstClosureExpr {
    /// `fn body(&self) -> Option<Expr>`
    fn body(&self) -> Option<AstExpr> {
        self.inner.body().map(|inner| AstExpr { inner })
    }
}
