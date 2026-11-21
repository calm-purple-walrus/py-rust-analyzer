use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstExpr, AstAwaitExpr};

#[pymethods]
impl AstExpr {
    /// The variant `Expr::AwaitExpr(AwaitExpr)`, read: its `AwaitExpr`, lent by this `Expr`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_AwaitExpr")]
    fn as_await_expr(slf: &Bound<'_, Self>) -> Option<AstAwaitExpr> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Expr`.
            ra_ap_syntax::ast::Expr::AwaitExpr(inner) => Some(AstAwaitExpr { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
