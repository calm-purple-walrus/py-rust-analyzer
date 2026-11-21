use pyo3::prelude::*;

use crate::types::{ModuleDef, Trait};

#[pymethods]
impl ModuleDef {
    /// The variant `ModuleDef::Trait(Trait)`, read: its `Trait` if the value is that variant, otherwise
    /// `None`.
    #[pyo3(name = "as_Trait")]
    fn as_trait(&self) -> Option<Trait> {
        match self.inner {
            ra_ap_hir::ModuleDef::Trait(inner) => Some(Trait { inner }),
            _ => None,
        }
    }
}
