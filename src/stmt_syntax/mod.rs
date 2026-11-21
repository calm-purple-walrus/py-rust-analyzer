use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstStmt, SyntaxNode};

#[pymethods]
impl AstStmt {
    /// `impl AstNode for Stmt`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Stmt`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::Stmt`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
