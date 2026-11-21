use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstParenExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::ParenExpr(ParenExpr)`, read: its `ParenExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_ParenExpr")]
    fn as_paren_expr(slf: &Bound<'_, Self>) -> Option<AstParenExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::ParenExpr(inner) => Some(AstParenExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
