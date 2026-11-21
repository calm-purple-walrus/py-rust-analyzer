use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstMeta, AstTokenTreeMeta};

#[pymethods]
impl AstMeta {
    /// The variant `Meta::TokenTreeMeta(TokenTreeMeta)`, read: its `TokenTreeMeta`, lent by this `Meta`, if the value is that variant,
    /// otherwise `None`.
    #[pyo3(name = "as_TokenTreeMeta")]
    fn as_token_tree_meta(slf: &Bound<'_, Self>) -> Option<AstTokenTreeMeta> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Meta`.
            ra_ap_syntax::ast::Meta::TokenTreeMeta(inner) => Some(AstTokenTreeMeta { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
