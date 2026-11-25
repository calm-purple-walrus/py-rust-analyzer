use pyo3::prelude::*;

use crate::types::Sysroot;

#[pymethods]
impl Sysroot {
    /// `pub fn error(&self) -> Option<&str>`
    fn error(&self) -> Option<&str> {
        self.inner.error()
    }
}
