use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstLoopExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::LoopExpr(LoopExpr)`, read: its `LoopExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_LoopExpr")]
    fn as_loop_expr(slf: &Bound<'_, Self>) -> Option<AstLoopExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::LoopExpr(inner) => Some(AstLoopExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
