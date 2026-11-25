use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::types::{AbsPathBuf, ProjectManifest};

#[pymethods]
impl ProjectManifest {
    /// `pub fn from_manifest_file(path: AbsPathBuf) -> anyhow::Result<ProjectManifest>`. It takes the path, so it
    /// is given a copy.
    #[staticmethod]
    fn from_manifest_file(path: &AbsPathBuf) -> PyResult<ProjectManifest> {
        ra_ap_project_model::ProjectManifest::from_manifest_file(path.inner.clone())
            .map(|inner| ProjectManifest { inner })
            .map_err(|error| PyRuntimeError::new_err(format!("{error:#}")))
    }
}
