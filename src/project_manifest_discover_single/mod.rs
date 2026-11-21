use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::types::{AbsPath, ProjectManifest};

#[pymethods]
impl ProjectManifest {
    /// `pub fn discover_single(path: &AbsPath) -> anyhow::Result<ProjectManifest>`
    #[staticmethod]
    fn discover_single(path: &AbsPath) -> PyResult<ProjectManifest> {
        ra_ap_project_model::ProjectManifest::discover_single(&path.inner)
            .map(|inner| ProjectManifest { inner })
            .map_err(|error| PyRuntimeError::new_err(format!("{error:#}")))
    }
}
