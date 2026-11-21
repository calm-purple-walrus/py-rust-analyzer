use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstTokenTree, SyntaxNode};

#[pymethods]
impl AstTokenTree {
    /// `impl AstNode for TokenTree`: `fn syntax(&self) -> &SyntaxNode`, lent by this `TokenTree`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::TokenTree`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
