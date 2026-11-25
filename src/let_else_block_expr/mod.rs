use crate::held::Held;
use crate::types::{AstBlockExpr, AstLetElse};
use pyo3::prelude::*;

#[pymethods]
impl AstLetElse {
    /// `fn block_expr(&self) -> Option<BlockExpr>`
    fn block_expr(&self) -> Option<AstBlockExpr> {
        self.inner.block_expr().map(|inner| AstBlockExpr {
            inner: Held::Own(inner),
        })
    }
}
