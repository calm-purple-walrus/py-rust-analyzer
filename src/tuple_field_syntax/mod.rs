use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstTupleField, SyntaxNode};

#[pymethods]
impl AstTupleField {
    /// `impl AstNode for TupleField`: `fn syntax(&self) -> &SyntaxNode`, lent by this `TupleField`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::TupleField`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
