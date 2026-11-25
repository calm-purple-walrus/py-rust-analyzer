use pyo3::prelude::*;

use crate::types::Adt;

#[pymethods]
impl Adt {
    /// `impl PartialEq for Adt`
    fn __eq__(&self, other: &Adt) -> bool {
        self.inner == other.inner
    }
}
