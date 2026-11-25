use pyo3::prelude::*;

use crate::types::Docs;

#[pymethods]
impl Docs {
    /// `pub fn docs(&self) -> &str`
    fn docs(&self) -> &str {
        self.inner.docs()
    }
}
