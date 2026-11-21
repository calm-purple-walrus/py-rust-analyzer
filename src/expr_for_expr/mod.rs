use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstForExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::ForExpr(ForExpr)`, read: its `ForExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_ForExpr")]
    fn as_for_expr(slf: &Bound<'_, Self>) -> Option<AstForExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::ForExpr(inner) => Some(AstForExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
