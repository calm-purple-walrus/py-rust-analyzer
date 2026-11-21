use pyo3::prelude::*;

use crate::types::{TextRange, TextSize};

#[pymethods]
impl TextRange {
    /// `pub const fn end(self) -> TextSize`
    fn end(&self) -> TextSize {
        TextSize { inner: self.inner.end() }
    }
}
