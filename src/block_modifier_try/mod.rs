use crate::held::{Held, owner};
use crate::types::{AstBlockModifier, AstType, SyntaxToken};
use pyo3::prelude::*;

#[pymethods]
impl AstBlockModifier {
    /// The `BlockModifier::Try` variant, read.
    #[pyo3(name = "as_Try")]
    fn as_try(
        slf: &Bound<'_, Self>,
    ) -> Option<(SyntaxToken, Option<SyntaxToken>, Option<AstType>)> {
        let this = slf.borrow();
        match &this.inner {
            ra_ap_syntax::ast::BlockModifier::Try {
                try_token,
                bikeshed_token,
                result_type,
            } => {
                // SAFETY: both tokens belong to this immutable block modifier.
                let token = |value| SyntaxToken {
                    inner: unsafe { Held::lent(value, owner(slf)) },
                };
                Some((
                    token(try_token),
                    bikeshed_token.as_ref().map(token),
                    result_type.clone().map(|inner| AstType { inner }),
                ))
            }
            _ => None,
        }
    }
}
