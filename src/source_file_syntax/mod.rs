use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{SourceFile, SyntaxNode};

#[pymethods]
impl SourceFile {
    /// `impl AstNode for SourceFile`: `fn syntax(&self) -> &SyntaxNode`, lent by this `SourceFile`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::SourceFile`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
