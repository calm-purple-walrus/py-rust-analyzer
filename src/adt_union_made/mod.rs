use pyo3::prelude::*;

use crate::types::{Adt, Union};

#[pymethods]
impl Adt {
    /// The variant `Adt::Union(Union)`, made.
    #[staticmethod]
    #[pyo3(name = "Union")]
    fn union_(value: &Union) -> Adt {
        Adt { inner: ra_ap_hir::Adt::Union(value.inner) }
    }
}
