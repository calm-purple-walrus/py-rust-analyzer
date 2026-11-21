use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstModule, SyntaxNode};

#[pymethods]
impl AstModule {
    /// `impl AstNode for Module`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Module`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes a `ast::Module`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
