use pyo3::prelude::*;

use crate::types::SyntaxText;

#[pymethods]
impl SyntaxText {
    /// `impl Display for SyntaxText`
    fn __str__(&self) -> String {
        self.inner.to_string()
    }
}
