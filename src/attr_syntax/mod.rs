use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstAttr, SyntaxNode};

#[pymethods]
impl AstAttr {
    /// `impl AstNode for Attr`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Attr`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::Attr`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
