use crate::types::{AstExpr, AstIfExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstIfExpr {
    /// `fn condition(&self) -> Option<Expr>`
    fn condition(&self) -> Option<AstExpr> {
        self.inner.condition().map(|inner| AstExpr { inner })
    }
}
