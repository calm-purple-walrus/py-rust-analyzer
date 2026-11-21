use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{SyntaxElement, SyntaxToken};

#[pymethods]
impl SyntaxElement {
    /// The variant `NodeOrToken::Token(T)`, read: its `SyntaxToken`, lent by this `SyntaxElement`, if the value
    /// is that variant, otherwise `None`.
    #[pyo3(name = "as_Token")]
    fn as_token(slf: &Bound<'_, Self>) -> Option<SyntaxToken> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the token lives in this object, and no binding changes a `SyntaxElement`.
            ra_ap_syntax::NodeOrToken::Token(token) => Some(SyntaxToken { inner: unsafe { Held::lent(token, owner(slf)) } }),
            ra_ap_syntax::NodeOrToken::Node(_) => None,
        }
    }
}
