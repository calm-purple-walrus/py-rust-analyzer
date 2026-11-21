use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstIfExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::IfExpr(IfExpr)`, read: its `IfExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_IfExpr")]
    fn as_if_expr(slf: &Bound<'_, Self>) -> Option<AstIfExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::IfExpr(inner) => Some(AstIfExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
