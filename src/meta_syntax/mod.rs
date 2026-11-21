use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstMeta, SyntaxNode};

#[pymethods]
impl AstMeta {
    /// `impl AstNode for Meta`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Meta`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::Meta`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
