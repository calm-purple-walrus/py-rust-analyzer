use pyo3::prelude::*;

use crate::types::{LoadCargoConfig, ProcMacroServerChoice};

#[pymethods]
impl LoadCargoConfig {
    /// `pub with_proc_macro_server: ProcMacroServerChoice`
    #[getter]
    fn with_proc_macro_server(&self) -> ProcMacroServerChoice {
        ProcMacroServerChoice { inner: self.inner.with_proc_macro_server.clone() }
    }

    #[setter]
    fn set_with_proc_macro_server(&mut self, value: &ProcMacroServerChoice) {
        self.inner.with_proc_macro_server = value.inner.clone();
    }
}
