use pyo3::prelude::*;

use crate::types::RustLibSource;

#[pymethods]
impl RustLibSource {
    /// The variant `RustLibSource::Discover`, made from Python.
    #[staticmethod]
    #[pyo3(name = "Discover")]
    fn discover() -> RustLibSource {
        RustLibSource { inner: ra_ap_project_model::RustLibSource::Discover }
    }
}
