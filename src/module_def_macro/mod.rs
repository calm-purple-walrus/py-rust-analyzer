use crate::types::{Macro, ModuleDef};
use pyo3::prelude::*;

#[pymethods]
impl ModuleDef {
    /// The `ModuleDef::Macro` variant, read.
    #[pyo3(name = "as_Macro")]
    fn as_macro(&self) -> Option<Macro> {
        match &self.inner {
            ra_ap_hir::ModuleDef::Macro(inner) => Some(Macro { inner: *inner }),
            _ => None,
        }
    }
}
