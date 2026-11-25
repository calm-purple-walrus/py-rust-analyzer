use crate::types::{AstArrayExpr, AstArrayExprKind};
use pyo3::prelude::*;

#[pymethods]
impl AstArrayExpr {
    /// `fn kind(&self) -> ArrayExprKind`
    fn kind(&self) -> AstArrayExprKind {
        AstArrayExprKind {
            inner: self.inner.kind(),
        }
    }
}
