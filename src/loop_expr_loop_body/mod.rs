use crate::held::Held;
use crate::types::{AstBlockExpr, AstLoopExpr};
use pyo3::prelude::*;
use ra_ap_syntax::ast::HasLoopBody;

#[pymethods]
impl AstLoopExpr {
    /// `fn loop_body(&self) -> Option<BlockExpr>`
    fn loop_body(&self) -> Option<AstBlockExpr> {
        self.inner.loop_body().map(|inner| AstBlockExpr {
            inner: Held::Own(inner),
        })
    }
}
