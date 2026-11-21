use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstPat, AstIdentPat};

#[pymethods]
impl AstPat {
    /// The variant `Pat::IdentPat(IdentPat)`, read: its `IdentPat`, lent by this `Pat`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_IdentPat")]
    fn as_ident_pat(slf: &Bound<'_, Self>) -> Option<AstIdentPat> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Pat`.
            ra_ap_syntax::ast::Pat::IdentPat(inner) => Some(AstIdentPat { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
