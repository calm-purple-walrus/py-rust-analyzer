use pyo3::prelude::*;

use crate::types::WideLineCol;

#[pymethods]
impl WideLineCol {
    /// `pub col: u32`
    #[getter]
    fn col(&self) -> u32 {
        self.inner.col
    }
}
