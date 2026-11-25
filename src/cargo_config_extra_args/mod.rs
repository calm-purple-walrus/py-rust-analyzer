use pyo3::prelude::*;

use crate::types::CargoConfig;

#[pymethods]
impl CargoConfig {
    /// `pub extra_args: Vec<String>`. Copied, because the setter changes the `CargoConfig`.
    #[getter]
    fn extra_args(&self) -> Vec<String> {
        self.inner.extra_args.clone()
    }

    #[setter]
    fn set_extra_args(&mut self, value: Vec<String>) {
        self.inner.extra_args = value;
    }
}
