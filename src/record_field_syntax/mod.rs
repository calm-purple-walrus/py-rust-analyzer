use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstRecordField, SyntaxNode};

#[pymethods]
impl AstRecordField {
    /// `impl AstNode for RecordField`: `fn syntax(&self) -> &SyntaxNode`, lent by this `RecordField`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::RecordField`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
