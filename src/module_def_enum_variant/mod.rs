use pyo3::prelude::*;

use crate::types::{EnumVariant, ModuleDef};

#[pymethods]
impl ModuleDef {
    /// The variant `ModuleDef::EnumVariant(EnumVariant)`, read: its `EnumVariant` if the value is that variant,
    /// otherwise `None`.
    #[pyo3(name = "as_EnumVariant")]
    fn as_enum_variant(&self) -> Option<EnumVariant> {
        match self.inner {
            ra_ap_hir::ModuleDef::EnumVariant(inner) => Some(EnumVariant { inner }),
            _ => None,
        }
    }
}
