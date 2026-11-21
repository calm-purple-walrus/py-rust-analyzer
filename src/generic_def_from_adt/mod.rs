use pyo3::prelude::*;

use crate::types::{Adt, GenericDef};

#[pymethods]
impl GenericDef {
    /// `impl From<Adt> for GenericDef`: `fn from(value: Adt) -> Self`
    #[staticmethod]
    #[pyo3(name = "from_Adt")]
    fn from_adt(adt: &Adt) -> GenericDef {
        GenericDef { inner: ra_ap_hir::GenericDef::from(adt.inner) }
    }
}
