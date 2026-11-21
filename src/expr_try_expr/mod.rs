use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstTryExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::TryExpr(TryExpr)`, read: its `TryExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_TryExpr")]
    fn as_try_expr(slf: &Bound<'_, Self>) -> Option<AstTryExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::TryExpr(inner) => Some(AstTryExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
