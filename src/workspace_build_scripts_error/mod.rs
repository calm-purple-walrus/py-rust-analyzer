use pyo3::prelude::*;

use crate::types::WorkspaceBuildScripts;

#[pymethods]
impl WorkspaceBuildScripts {
    /// `pub fn error(&self) -> Option<&str>`
    fn error(&self) -> Option<&str> {
        self.inner.error()
    }
}
