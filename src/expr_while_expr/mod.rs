use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstWhileExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::WhileExpr(WhileExpr)`, read: its `WhileExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_WhileExpr")]
    fn as_while_expr(slf: &Bound<'_, Self>) -> Option<AstWhileExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::WhileExpr(inner) => Some(AstWhileExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
