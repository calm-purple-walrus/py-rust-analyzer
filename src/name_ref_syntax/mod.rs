use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstNameRef, SyntaxNode};

#[pymethods]
impl AstNameRef {
    /// `impl AstNode for NameRef`: `fn syntax(&self) -> &SyntaxNode`, lent by this `NameRef`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::NameRef`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
