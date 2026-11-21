use pyo3::prelude::*;

use crate::types::LoadCargoConfig;

#[pymethods]
impl LoadCargoConfig {
    /// `pub proc_macro_processes: usize`
    #[getter]
    fn proc_macro_processes(&self) -> usize {
        self.inner.proc_macro_processes
    }

    #[setter]
    fn set_proc_macro_processes(&mut self, value: usize) {
        self.inner.proc_macro_processes = value;
    }
}
