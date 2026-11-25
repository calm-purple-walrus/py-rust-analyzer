use pyo3::prelude::*;

use crate::types::{GenericDef, Trait};

#[pymethods]
impl GenericDef {
    /// `impl From<Trait> for GenericDef`: `fn from(value: Trait) -> Self`
    #[staticmethod]
    #[pyo3(name = "from_Trait")]
    fn from_trait(value: &Trait) -> GenericDef {
        GenericDef { inner: ra_ap_hir::GenericDef::from(value.inner) }
    }
}
