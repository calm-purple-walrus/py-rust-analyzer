use pyo3::prelude::*;

use crate::held::{Held, owner};
use crate::types::{AstMeta, AstKeyValueMeta};

#[pymethods]
impl AstMeta {
    /// The variant `Meta::KeyValueMeta(KeyValueMeta)`, read: its `KeyValueMeta`, lent by this `Meta`, if the value is that variant,
    /// otherwise `None`.
    #[pyo3(name = "as_KeyValueMeta")]
    fn as_key_value_meta(slf: &Bound<'_, Self>) -> Option<AstKeyValueMeta> {
        let this = slf.borrow();
        match &this.inner {
            // SAFETY: the value lives in this object, and no binding changes an `ast::Meta`.
            ra_ap_syntax::ast::Meta::KeyValueMeta(inner) => Some(AstKeyValueMeta { inner: unsafe { Held::lent(inner, owner(slf)) } }),
            _ => None,
        }
    }
}
