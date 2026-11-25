use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstCfgMeta, AstMeta};

#[pymethods]
impl AstMeta {
    /// The variant `Meta::CfgMeta(CfgMeta)`, read: its `CfgMeta`, lent by this `Meta`, if the value is that
    /// variant, otherwise `None`.
    #[pyo3(name = "as_CfgMeta")]
    fn as_cfg_meta(slf: &Bound<'_, Self>) -> Option<AstCfgMeta> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Meta`.
            ra_ap_syntax::ast::Meta::CfgMeta(inner) => Some(AstCfgMeta { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
