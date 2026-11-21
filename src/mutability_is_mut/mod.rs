use pyo3::prelude::*;

use crate::types::Mutability;

#[pymethods]
impl Mutability {
    /// `pub fn is_mut(&self) -> bool`
    fn is_mut(&self) -> bool {
        self.inner.is_mut()
    }
}
