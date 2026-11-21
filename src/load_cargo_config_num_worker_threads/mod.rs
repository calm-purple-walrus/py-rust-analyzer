use pyo3::prelude::*;

use crate::types::LoadCargoConfig;

#[pymethods]
impl LoadCargoConfig {
    /// `pub num_worker_threads: usize`
    #[getter]
    fn num_worker_threads(&self) -> usize {
        self.inner.num_worker_threads
    }

    #[setter]
    fn set_num_worker_threads(&mut self, value: usize) {
        self.inner.num_worker_threads = value;
    }
}
