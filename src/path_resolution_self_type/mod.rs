use pyo3::prelude::*;

use crate::types::{Impl, PathResolution};

#[pymethods]
impl PathResolution {
    /// The variant `PathResolution::SelfType(Impl)`, read: its `Impl` if the value is that variant, otherwise
    /// `None`.
    #[pyo3(name = "as_SelfType")]
    fn as_self_type(&self) -> Option<Impl> {
        match self.inner {
            ra_ap_hir::PathResolution::SelfType(inner) => Some(Impl { inner }),
            _ => None,
        }
    }
}
