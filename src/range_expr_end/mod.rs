use crate::types::{AstExpr, AstRangeExpr};
use pyo3::prelude::*;
use ra_ap_syntax::ast::RangeItem;

#[pymethods]
impl AstRangeExpr {
    /// `fn end(&self) -> Option<Expr>`
    fn end(&self) -> Option<AstExpr> {
        self.inner.end().map(|inner| AstExpr { inner })
    }
}
