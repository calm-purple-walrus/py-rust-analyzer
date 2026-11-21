use pyo3::prelude::*;

use crate::types::Visibility;

#[pymethods]
impl Visibility {
    /// `impl PartialEq for Visibility`
    fn __eq__(&self, other: &Visibility) -> bool {
        self.inner == other.inner
    }
}
