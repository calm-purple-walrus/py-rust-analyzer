use pyo3::prelude::*;

use crate::types::LineCol;

#[pymethods]
impl LineCol {
    /// `pub col: u32`
    #[getter]
    fn col(&self) -> u32 {
        self.inner.col
    }
}
