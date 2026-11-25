use crate::held::Held;
use crate::types::{AstRangePat, SyntaxToken};
use pyo3::prelude::*;
use ra_ap_syntax::ast::RangeItem;

#[pymethods]
impl AstRangePat {
    /// `RangeItem::op_token(&self) -> Option<SyntaxToken>`
    fn op_token(&self) -> Option<SyntaxToken> {
        self.inner.op_token().map(|inner| SyntaxToken {
            inner: Held::Own(inner),
        })
    }
}
