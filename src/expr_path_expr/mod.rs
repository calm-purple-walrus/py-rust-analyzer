use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstPathExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::PathExpr(PathExpr)`, read: its `PathExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_PathExpr")]
    fn as_path_expr(slf: &Bound<'_, Self>) -> Option<AstPathExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::PathExpr(inner) => Some(AstPathExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
