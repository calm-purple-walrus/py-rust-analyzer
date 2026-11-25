use crate::types::{AstExpr, AstRangeExpr};
use pyo3::prelude::*;
use ra_ap_syntax::ast::RangeItem;

#[pymethods]
impl AstRangeExpr {
    /// `fn start(&self) -> Option<Expr>`
    fn start(&self) -> Option<AstExpr> {
        self.inner.start().map(|inner| AstExpr { inner })
    }
}
