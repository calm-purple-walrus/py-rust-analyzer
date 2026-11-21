use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstCfgPredicate, SyntaxNode};

#[pymethods]
impl AstCfgPredicate {
    /// `impl AstNode for CfgPredicate`: `fn syntax(&self) -> &SyntaxNode`, lent by this `CfgPredicate`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in this object, and no binding changes an `ast::CfgPredicate`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
