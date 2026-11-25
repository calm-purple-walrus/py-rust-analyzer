use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstImpl, SyntaxNode};

#[pymethods]
impl AstImpl {
    /// `impl AstNode for Impl`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Impl`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in the value this object holds or borrows, and no binding changes an
        // `ast::Impl`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
