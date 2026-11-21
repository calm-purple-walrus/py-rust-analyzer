use pyo3::prelude::*;

use crate::types::LoadCargoConfig;

#[pymethods]
impl LoadCargoConfig {
    /// `pub prefill_caches: bool`
    #[getter]
    fn prefill_caches(&self) -> bool {
        self.inner.prefill_caches
    }

    #[setter]
    fn set_prefill_caches(&mut self, value: bool) {
        self.inner.prefill_caches = value;
    }
}
