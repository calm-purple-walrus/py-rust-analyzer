use pyo3::prelude::*;

use crate::types::{Struct, Variant};

#[pymethods]
impl Variant {
    /// The variant `Variant::Struct(Struct)`, read: its `Struct` if the value is that variant, otherwise `None`.
    #[pyo3(name = "as_Struct")]
    fn as_struct(&self) -> Option<Struct> {
        match self.inner {
            ra_ap_hir::Variant::Struct(inner) => Some(Struct { inner }),
            _ => None,
        }
    }
}
