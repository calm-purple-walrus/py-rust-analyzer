use pyo3::prelude::*;

use crate::types::{FileRange, TextRange};

#[pymethods]
impl FileRange {
    /// `pub range: TextRange`
    #[getter]
    fn range(&self) -> TextRange {
        TextRange { inner: self.inner.range }
    }
}
