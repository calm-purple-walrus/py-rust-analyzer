use pyo3::prelude::*;

use crate::held::Held;
use crate::types::{AstFn, AstBlockExpr};

#[pymethods]
impl AstFn {
    /// `pub fn body(&self) -> Option<BlockExpr>`
    fn body(&self) -> Option<AstBlockExpr> {
        self.inner.body().map(|inner| AstBlockExpr { inner: Held::Own(inner) })
    }
}
