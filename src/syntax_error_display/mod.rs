use pyo3::prelude::*;

use crate::types::SyntaxError;

#[pymethods]
impl SyntaxError {
    /// `impl Display for SyntaxError`
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}
