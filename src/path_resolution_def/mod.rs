use pyo3::prelude::*;

use crate::types::{ModuleDef, PathResolution};

#[pymethods]
impl PathResolution {
    /// The variant `PathResolution::Def(ModuleDef)`, read: its `ModuleDef` if the value is that variant, otherwise
    /// `None`.
    #[pyo3(name = "as_Def")]
    fn as_def(&self) -> Option<ModuleDef> {
        match self.inner {
            ra_ap_hir::PathResolution::Def(inner) => Some(ModuleDef { inner }),
            _ => None,
        }
    }
}
