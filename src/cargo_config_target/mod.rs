use pyo3::prelude::*;

use crate::types::CargoConfig;

#[pymethods]
impl CargoConfig {
    /// `pub target: Option<String>`. Copied, because the setter changes the `CargoConfig`.
    #[getter]
    fn target(&self) -> Option<String> {
        self.inner.target.clone()
    }

    #[setter]
    fn set_target(&mut self, value: Option<String>) {
        self.inner.target = value;
    }
}
