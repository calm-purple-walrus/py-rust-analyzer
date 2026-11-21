use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstBlockExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::BlockExpr(BlockExpr)`, read: its `BlockExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_BlockExpr")]
    fn as_block_expr(slf: &Bound<'_, Self>) -> Option<AstBlockExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::BlockExpr(inner) => Some(AstBlockExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
