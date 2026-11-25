use pyo3::prelude::*;

use crate::types::Module;

#[pymethods]
impl Module {
    /// `impl PartialEq for Module`
    fn __eq__(&self, other: &Module) -> bool {
        self.inner == other.inner
    }
}
