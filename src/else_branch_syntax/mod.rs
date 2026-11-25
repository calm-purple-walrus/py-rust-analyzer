use crate::held::{Held, owner};
use crate::types::{AstElseBranch, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstElseBranch {
    /// `fn syntax(&self) -> &SyntaxNode`, lent by the receiver.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the syntax node belongs to this immutable AstElseBranch.
        SyntaxNode {
            inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) },
        }
    }
}
