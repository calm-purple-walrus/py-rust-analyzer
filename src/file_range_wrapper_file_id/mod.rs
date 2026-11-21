use pyo3::prelude::*;

use crate::types::{EditionedFileId, FileRange};

#[pymethods]
impl FileRange {
    /// `pub file_id: EditionedFileId` of `FileRangeWrapper<EditionedFileId>`
    #[getter]
    fn file_id(&self) -> EditionedFileId {
        EditionedFileId { inner: self.inner.file_id }
    }
}
