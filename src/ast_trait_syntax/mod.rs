use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::held::{Held, owner};
use crate::types::{AstTrait, SyntaxNode};

#[pymethods]
impl AstTrait {
    /// `impl AstNode for Trait`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Trait`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxNode {
        let this = slf.borrow();
        // SAFETY: the node lives in the value this object holds or borrows, and no binding changes an
        // `ast::Trait`.
        SyntaxNode { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
