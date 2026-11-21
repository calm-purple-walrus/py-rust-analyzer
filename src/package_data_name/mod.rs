use pyo3::prelude::*;

use crate::types::PackageData;

#[pymethods]
impl PackageData {
    /// `pub name: String`
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }
}
