use pyo3::prelude::*;

use crate::types::{HirFileId, InFileAstAdt, InFileFn, InFileSmallVecTextRange, InFileSyntaxNode, InFileTextRange};

// `pub file_id: FileKind` of `InFileWrapper<FileKind, T>`, for each `T` Python uses.

#[pymethods]
impl InFileAstAdt {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileFn {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileTextRange {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileSyntaxNode {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileSmallVecTextRange {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}
