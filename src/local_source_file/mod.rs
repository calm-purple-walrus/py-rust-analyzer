use crate::types::{HirFileId, LocalSource};
use pyo3::prelude::*;

#[pymethods]
impl LocalSource {
    /// `fn file(&self) -> HirFileId`
    fn file(&self) -> HirFileId {
        HirFileId {
            inner: self.inner.file(),
        }
    }
}
