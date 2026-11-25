use crate::types::{Module, ModuleDef};
use pyo3::prelude::*;

#[pymethods]
impl ModuleDef {
    /// The `ModuleDef::Module` variant, read.
    #[pyo3(name = "as_Module")]
    fn as_module(&self) -> Option<Module> {
        match &self.inner {
            ra_ap_hir::ModuleDef::Module(inner) => Some(Module { inner: *inner }),
            _ => None,
        }
    }
}
