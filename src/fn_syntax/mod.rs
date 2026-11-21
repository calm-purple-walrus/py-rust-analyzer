use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstFn, SyntaxNode};

#[pymethods]
impl AstFn {
    /// `impl AstNode for Fn`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Fn`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes a `ast::Fn`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
