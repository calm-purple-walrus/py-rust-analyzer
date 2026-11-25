use pyo3::prelude::*;

use crate::types::Trait;

#[pymethods]
impl Trait {
    /// `impl PartialEq for Trait`
    fn __eq__(&self, other: &Trait) -> bool {
        self.inner == other.inner
    }
}
