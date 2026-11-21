use pyo3::prelude::*;

use crate::types::SyntaxToken;

#[pymethods]
impl SyntaxToken {
    /// `pub fn text(&self) -> &str`
    fn text(&self) -> String {
        self.inner.text().to_owned()
    }
}
