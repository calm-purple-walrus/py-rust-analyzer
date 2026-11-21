use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstAssocItem, SyntaxNode};

#[pymethods]
impl AstAssocItem {
    /// `impl AstNode for AssocItem`: `fn syntax(&self) -> &SyntaxNode`, lent by this `AssocItem`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::AssocItem`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
