use pyo3::prelude::*;

use crate::types::{Trait, ModuleDef};

#[pymethods]
impl ModuleDef {
    /// The variant `ModuleDef::Trait(Trait)`, made.
    #[staticmethod]
    #[pyo3(name = "Trait")]
    fn trait_(value: &Trait) -> ModuleDef {
        ModuleDef { inner: ra_ap_hir::ModuleDef::Trait(value.inner) }
    }
}
