use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstAdt, SyntaxNode};

#[pymethods]
impl AstAdt {
    /// `impl AstNode for Adt`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Adt`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::Adt`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
