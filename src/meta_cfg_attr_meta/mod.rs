use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstMeta, AstCfgAttrMeta};

#[pymethods]
impl AstMeta {
    /// The variant `Meta::CfgAttrMeta(CfgAttrMeta)`, read: its `CfgAttrMeta`, lent by this `Meta`, if the value is that variant,
    /// otherwise `None`.
    #[pyo3(name = "as_CfgAttrMeta")]
    fn as_cfg_attr_meta(slf: &Bound<'_, Self>) -> Option<AstCfgAttrMeta> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Meta`.
            ra_ap_syntax::ast::Meta::CfgAttrMeta(inner) => Some(AstCfgAttrMeta { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
