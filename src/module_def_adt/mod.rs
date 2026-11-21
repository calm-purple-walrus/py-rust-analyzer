use pyo3::prelude::*;

use crate::types::{Adt, ModuleDef};

#[pymethods]
impl ModuleDef {
    /// The variant `ModuleDef::Adt(Adt)`, read: its `Adt` if the value is that variant, otherwise `None`.
    #[pyo3(name = "as_Adt")]
    fn as_adt(&self) -> Option<Adt> {
        match self.inner {
            ra_ap_hir::ModuleDef::Adt(inner) => Some(Adt { inner }),
            _ => None,
        }
    }
}
