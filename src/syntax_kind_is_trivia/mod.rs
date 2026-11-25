use pyo3::prelude::*;

use crate::types::SyntaxKind;

#[pymethods]
impl SyntaxKind {
    /// `pub fn is_trivia(self) -> bool`
    fn is_trivia(&self) -> bool {
        self.inner.is_trivia()
    }
}
