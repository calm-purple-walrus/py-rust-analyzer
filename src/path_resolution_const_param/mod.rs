use crate::types::{ConstParam, PathResolution};
use pyo3::prelude::*;

#[pymethods]
impl PathResolution {
    /// The `PathResolution::ConstParam` variant, read.
    #[pyo3(name = "as_ConstParam")]
    fn as_const_param(&self) -> Option<ConstParam> {
        match &self.inner {
            ra_ap_hir::PathResolution::ConstParam(inner) => Some(ConstParam { inner: *inner }),
            _ => None,
        }
    }
}
