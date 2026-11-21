use pyo3::prelude::*;

use crate::types::{Adt, ModuleDef};

#[pymethods]
impl ModuleDef {
    /// The variant `ModuleDef::Adt(Adt)`, made.
    #[staticmethod]
    #[pyo3(name = "Adt")]
    fn adt(value: &Adt) -> ModuleDef {
        ModuleDef { inner: ra_ap_hir::ModuleDef::Adt(value.inner) }
    }
}
