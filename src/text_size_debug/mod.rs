use pyo3::prelude::*;

use crate::types::TextSize;

#[pymethods]
impl TextSize {
    /// `impl Debug for TextSize`
    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}
