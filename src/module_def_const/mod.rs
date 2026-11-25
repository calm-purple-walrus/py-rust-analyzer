use crate::types::{Const, ModuleDef};
use pyo3::prelude::*;

#[pymethods]
impl ModuleDef {
    /// The `ModuleDef::Const` variant, read.
    #[pyo3(name = "as_Const")]
    fn as_const(&self) -> Option<Const> {
        match &self.inner {
            ra_ap_hir::ModuleDef::Const(inner) => Some(Const { inner: *inner }),
            _ => None,
        }
    }
}
