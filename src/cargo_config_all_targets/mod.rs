use pyo3::prelude::*;

use crate::types::CargoConfig;

#[pymethods]
impl CargoConfig {
    /// `pub all_targets: bool`
    #[getter]
    fn all_targets(&self) -> bool {
        self.inner.all_targets
    }

    #[setter]
    fn set_all_targets(&mut self, value: bool) {
        self.inner.all_targets = value;
    }
}
