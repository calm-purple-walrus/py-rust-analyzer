use pyo3::prelude::*;

use crate::types::{EditionedFileId, HirFileId};

#[pymethods]
impl HirFileId {
    /// `pub fn file_id(self) -> Option<EditionedFileId>`
    fn file_id(&self) -> Option<EditionedFileId> {
        self.inner.file_id().map(|inner| EditionedFileId { inner })
    }
}
