use pyo3::prelude::*;

use crate::types::{EnumVariant, Variant};

#[pymethods]
impl Variant {
    /// The variant `Variant::EnumVariant(EnumVariant)`, read: its `EnumVariant` if the value is that variant,
    /// otherwise `None`.
    #[pyo3(name = "as_EnumVariant")]
    fn as_enum_variant(&self) -> Option<EnumVariant> {
        match self.inner {
            ra_ap_hir::Variant::EnumVariant(inner) => Some(EnumVariant { inner }),
            _ => None,
        }
    }
}
