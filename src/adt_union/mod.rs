use pyo3::prelude::*;

use crate::types::{Adt, Union};

#[pymethods]
impl Adt {
    /// The variant `Adt::Union(Union)`, read: its `Union` if the value is that variant, otherwise `None`.
    #[pyo3(name = "as_Union")]
    fn as_union(&self) -> Option<Union> {
        match self.inner {
            ra_ap_hir::Adt::Union(inner) => Some(Union { inner }),
            _ => None,
        }
    }
}
