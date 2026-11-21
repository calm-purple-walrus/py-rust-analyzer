use pyo3::prelude::*;

use crate::types::PackageDependency;

#[pymethods]
impl PackageDependency {
    /// `pub name: String`
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }
}
