use pyo3::prelude::*;

use crate::types::SyntaxKind;

#[pymethods]
impl SyntaxKind {
    /// `impl PartialEq for SyntaxKind`
    fn __eq__(&self, other: &SyntaxKind) -> bool {
        self.inner == other.inner
    }
}
