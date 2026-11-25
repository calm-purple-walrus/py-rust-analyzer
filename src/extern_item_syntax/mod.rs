use crate::held::{Held, owner};
use crate::types::{AstExternItem, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstExternItem {
    /// `fn syntax(&self) -> &SyntaxNode`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the syntax node belongs to this immutable AstExternItem.
        SyntaxNode {
            inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) },
        }
    }
}
