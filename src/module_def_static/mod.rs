use crate::types::{ModuleDef, Static};
use pyo3::prelude::*;

#[pymethods]
impl ModuleDef {
    /// The `ModuleDef::Static` variant, read.
    #[pyo3(name = "as_Static")]
    fn as_static(&self) -> Option<Static> {
        match &self.inner {
            ra_ap_hir::ModuleDef::Static(inner) => Some(Static { inner: *inner }),
            _ => None,
        }
    }
}
