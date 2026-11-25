use crate::held::{Held, owner};
use crate::types::{AstFieldKind, SyntaxToken};
use pyo3::prelude::*;

#[pymethods]
impl AstFieldKind {
    /// The `FieldKind::Index` variant, read.
    #[pyo3(name = "as_Index")]
    fn as_index(slf: &Bound<'_, Self>) -> Option<SyntaxToken> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the token belongs to this immutable field kind.
            ra_ap_syntax::ast::FieldKind::Index(inner) => Some(SyntaxToken {
                inner: unsafe { Held::lent(inner, owner(slf)) },
            }),
            _ => None,
        }
    }
}
