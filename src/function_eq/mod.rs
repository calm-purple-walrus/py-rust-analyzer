use pyo3::prelude::*;

use crate::types::Function;

#[pymethods]
impl Function {
    /// `impl PartialEq for Function`
    fn __eq__(&self, other: &Function) -> bool {
        self.inner == other.inner
    }
}
