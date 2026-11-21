use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstBlockExpr, SyntaxNode};

#[pymethods]
impl AstBlockExpr {
    /// `impl AstNode for BlockExpr`: `fn syntax(&self) -> &SyntaxNode`, lent by this `BlockExpr`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::BlockExpr`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
