use pyo3::prelude::*;

use crate::types::{Package, TargetData};

#[pymethods]
impl TargetData {
    /// `pub package: Package`
    #[getter]
    fn package(&self) -> Package {
        Package { inner: self.inner.package }
    }
}
