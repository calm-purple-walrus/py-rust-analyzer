use crate::held::Held;
use crate::types::{AstBlockExpr, AstIfExpr};
use pyo3::prelude::*;

#[pymethods]
impl AstIfExpr {
    /// `fn then_branch(&self) -> Option<BlockExpr>`
    fn then_branch(&self) -> Option<AstBlockExpr> {
        self.inner.then_branch().map(|inner| AstBlockExpr {
            inner: Held::Own(inner),
        })
    }
}
