use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::types::{CargoConfig, ProjectWorkspace, WorkspaceBuildScripts};

#[pymethods]
impl ProjectWorkspace {
    /// `pub fn run_build_scripts(&self, config: &CargoConfig, progress: &dyn Fn(String)) -> anyhow::Result<WorkspaceBuildScripts>`
    ///
    /// `progress` is a Python callable that takes a string. If it raises, the binding panics.
    fn run_build_scripts(
        &self,
        py: Python<'_>,
        config: &CargoConfig,
        progress: Py<PyAny>,
    ) -> PyResult<WorkspaceBuildScripts> {
        let report = |message: String| {
            if let Err(error) = progress.call1(py, (message,)) {
                panic!("the progress callback raised {error}");
            }
        };
        self.inner
            .run_build_scripts(&config.inner, &report)
            .map(|inner| WorkspaceBuildScripts { inner })
            .map_err(|error| PyRuntimeError::new_err(format!("{error:#}")))
    }
}
