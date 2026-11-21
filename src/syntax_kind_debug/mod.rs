use pyo3::prelude::*;

use crate::types::SyntaxKind;

#[pymethods]
impl SyntaxKind {
    /// `impl Debug for SyntaxKind`
    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}
