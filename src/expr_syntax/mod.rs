use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstExpr, SyntaxNode};

#[pymethods]
impl AstExpr {
    /// `impl AstNode for Expr`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Expr`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::Expr`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
