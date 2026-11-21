use pyo3::prelude::*;

use crate::types::{Adt, Enum};

#[pymethods]
impl Adt {
    /// The variant `Adt::Enum(Enum)`, made.
    #[staticmethod]
    #[pyo3(name = "Enum")]
    fn enum_(value: &Enum) -> Adt {
        Adt { inner: ra_ap_hir::Adt::Enum(value.inner) }
    }
}
