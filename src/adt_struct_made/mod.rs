use pyo3::prelude::*;

use crate::types::{Adt, Struct};

#[pymethods]
impl Adt {
    /// The variant `Adt::Struct(Struct)`, made.
    #[staticmethod]
    #[pyo3(name = "Struct")]
    fn struct_(value: &Struct) -> Adt {
        Adt { inner: ra_ap_hir::Adt::Struct(value.inner) }
    }
}
