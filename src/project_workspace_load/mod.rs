use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::types::{CargoConfig, ProjectManifest, ProjectWorkspace};

#[pymethods]
impl ProjectWorkspace {
    /// `pub fn load(manifest: ProjectManifest, config: &CargoConfig, progress: &(dyn Fn(String) + Sync)) -> anyhow::Result<ProjectWorkspace>`
    ///
    /// `progress` is a Python callable that takes a string. If it raises, the binding panics.
    #[staticmethod]
    fn load(
        py: Python<'_>,
        manifest: &ProjectManifest,
        config: &CargoConfig,
        progress: Py<PyAny>,
    ) -> PyResult<ProjectWorkspace> {
        let manifest = manifest.inner.clone();
        let report = |message: String| {
            Python::attach(|py| {
                if let Err(error) = progress.call1(py, (message,)) {
                    panic!("the progress callback raised {error}");
                }
            })
        };
        // rust-analyzer calls `progress` from a thread of its own (`fetch_metadata.exec` in
        // `ProjectWorkspace::load_cargo`), which can only run Python while this thread lets go of it.
        let loaded = py.detach(|| ra_ap_project_model::ProjectWorkspace::load(manifest, &config.inner, &report));
        loaded.map(|inner| ProjectWorkspace { inner }).map_err(|error| PyRuntimeError::new_err(format!("{error:#}")))
    }
}
