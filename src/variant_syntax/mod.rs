use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstVariant, SyntaxNode};

#[pymethods]
impl AstVariant {
    /// `impl AstNode for Variant`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Variant`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::Variant`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
