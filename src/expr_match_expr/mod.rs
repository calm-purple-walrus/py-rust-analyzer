use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstMatchExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::MatchExpr(MatchExpr)`, read: its `MatchExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_MatchExpr")]
    fn as_match_expr(slf: &Bound<'_, Self>) -> Option<AstMatchExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::MatchExpr(inner) => Some(AstMatchExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
