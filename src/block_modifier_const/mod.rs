use crate::held::{Held, owner};
use crate::types::{AstBlockModifier, SyntaxToken};
use pyo3::prelude::*;

#[pymethods]
impl AstBlockModifier {
    /// The `BlockModifier::Const` variant, read.
    #[pyo3(name = "as_Const")]
    fn as_const(slf: &Bound<'_, Self>) -> Option<SyntaxToken> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the token belongs to this immutable block modifier.
            ra_ap_syntax::ast::BlockModifier::Const(inner) => Some(SyntaxToken {
                inner: unsafe { Held::lent(inner, owner(slf)) },
            }),
            _ => None,
        }
    }
}
