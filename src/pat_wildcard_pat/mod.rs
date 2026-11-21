use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstPat, AstWildcardPat};

#[pymethods]
impl AstPat {
    /// The variant `Pat::WildcardPat(WildcardPat)`, read: its `WildcardPat`, lent by this `Pat`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_WildcardPat")]
    fn as_wildcard_pat(slf: &Bound<'_, Self>) -> Option<AstWildcardPat> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Pat`.
            ra_ap_syntax::ast::Pat::WildcardPat(inner) => Some(AstWildcardPat { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
