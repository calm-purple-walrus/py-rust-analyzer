use crate::types::{ModuleDef, TypeAlias};
use pyo3::prelude::*;

#[pymethods]
impl ModuleDef {
    /// The `ModuleDef::TypeAlias` variant, read.
    #[pyo3(name = "as_TypeAlias")]
    fn as_type_alias(&self) -> Option<TypeAlias> {
        match &self.inner {
            ra_ap_hir::ModuleDef::TypeAlias(inner) => Some(TypeAlias { inner: *inner }),
            _ => None,
        }
    }
}
