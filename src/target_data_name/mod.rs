use pyo3::prelude::*;

use crate::types::TargetData;

#[pymethods]
impl TargetData {
    /// `pub name: String`
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }
}
