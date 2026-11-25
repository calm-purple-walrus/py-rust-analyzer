use crate::types::{PathResolution, TypeParam};
use pyo3::prelude::*;

#[pymethods]
impl PathResolution {
    /// The `PathResolution::TypeParam` variant, read.
    #[pyo3(name = "as_TypeParam")]
    fn as_type_param(&self) -> Option<TypeParam> {
        match &self.inner {
            ra_ap_hir::PathResolution::TypeParam(inner) => Some(TypeParam { inner: *inner }),
            _ => None,
        }
    }
}
