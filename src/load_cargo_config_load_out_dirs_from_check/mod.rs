use pyo3::prelude::*;

use crate::types::LoadCargoConfig;

#[pymethods]
impl LoadCargoConfig {
    /// `pub load_out_dirs_from_check: bool`
    #[getter]
    fn load_out_dirs_from_check(&self) -> bool {
        self.inner.load_out_dirs_from_check
    }

    #[setter]
    fn set_load_out_dirs_from_check(&mut self, value: bool) {
        self.inner.load_out_dirs_from_check = value;
    }
}
