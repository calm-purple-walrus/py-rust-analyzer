use pyo3::prelude::*;

use crate::types::AstName;

#[pymethods]
impl AstName {
    /// `pub fn text(&self) -> &str`
    fn text(&self) -> String {
        self.inner.text().to_owned()
    }
}
