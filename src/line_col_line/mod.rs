use pyo3::prelude::*;

use crate::types::LineCol;

#[pymethods]
impl LineCol {
    /// `pub line: u32`
    #[getter]
    fn line(&self) -> u32 {
        self.inner.line
    }
}
