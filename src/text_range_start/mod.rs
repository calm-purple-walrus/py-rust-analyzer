use pyo3::prelude::*;

use crate::types::{TextRange, TextSize};

#[pymethods]
impl TextRange {
    /// `pub const fn start(self) -> TextSize`
    fn start(&self) -> TextSize {
        TextSize { inner: self.inner.start() }
    }
}
