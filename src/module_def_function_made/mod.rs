use crate::types::{Function, ModuleDef};
use pyo3::prelude::*;

#[pymethods]
impl ModuleDef {
    /// The `ModuleDef::Function` variant, made.
    #[staticmethod]
    #[pyo3(name = "Function")]
    fn function(value: &Function) -> ModuleDef {
        ModuleDef {
            inner: ra_ap_hir::ModuleDef::Function(value.inner),
        }
    }
}
