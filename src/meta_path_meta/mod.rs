use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstMeta, AstPathMeta};

#[pymethods]
impl AstMeta {
    /// The variant `Meta::PathMeta(PathMeta)`, read: its `PathMeta`, lent by this `Meta`, if the value is that variant,
    /// otherwise `None`.
    #[pyo3(name = "as_PathMeta")]
    fn as_path_meta(slf: &Bound<'_, Self>) -> Option<AstPathMeta> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Meta`.
            ra_ap_syntax::ast::Meta::PathMeta(inner) => Some(AstPathMeta { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
