use crate::held::Held;
use crate::types::{AstBlockExpr, AstForExpr};
use pyo3::prelude::*;
use ra_ap_syntax::ast::HasLoopBody;

#[pymethods]
impl AstForExpr {
    /// `fn loop_body(&self) -> Option<BlockExpr>`
    fn loop_body(&self) -> Option<AstBlockExpr> {
        self.inner.loop_body().map(|inner| AstBlockExpr {
            inner: Held::Own(inner),
        })
    }
}
