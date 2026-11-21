use pyo3::prelude::*;

use crate::types::CargoConfig;

#[pymethods]
impl CargoConfig {
    /// `pub set_test: bool`
    #[getter]
    fn set_test(&self) -> bool {
        self.inner.set_test
    }

    #[setter]
    fn set_set_test(&mut self, value: bool) {
        self.inner.set_test = value;
    }
}
