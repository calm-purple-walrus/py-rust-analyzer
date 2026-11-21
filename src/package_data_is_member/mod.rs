use pyo3::prelude::*;

use crate::types::PackageData;

#[pymethods]
impl PackageData {
    /// `pub is_member: bool`
    #[getter]
    fn is_member(&self) -> bool {
        self.inner.is_member
    }
}
