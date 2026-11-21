use pyo3::prelude::*;
use ra_ap_syntax::AstToken;

use crate::held::{Held, owner};
use crate::types::{AstComment, SyntaxToken};

#[pymethods]
impl AstComment {
    /// `impl AstToken for Comment`: `fn syntax(&self) -> &SyntaxToken`, lent by this `Comment`.
    fn syntax(slf: &Bound<'_, Self>) -> SyntaxToken {
        let this = slf.borrow();
        // SAFETY: the token lives in this object, and no binding changes an `ast::Comment`.
        SyntaxToken { inner: unsafe { Held::lent(this.inner.syntax(), owner(slf)) } }
    }
}
