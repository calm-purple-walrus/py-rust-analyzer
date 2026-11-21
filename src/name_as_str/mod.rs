use pyo3::prelude::*;

use crate::types::Name;

#[pymethods]
impl Name {
    /// `pub fn as_str(&self) -> &str`
    fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}
