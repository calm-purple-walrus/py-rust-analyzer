use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstMacroCall, SyntaxNode};

#[pymethods]
impl AstMacroCall {
    /// `impl AstNode for MacroCall`: `fn syntax(&self) -> &SyntaxNode`, lent by this `MacroCall`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes a `ast::MacroCall`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
