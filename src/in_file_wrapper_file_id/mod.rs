use pyo3::prelude::*;

use crate::types::{
    InRealFileSyntaxNode, EditionedFileId,
    HirFileId, InFileAstAdt, InFileAstImpl, InFileAstModule, InFileAstTrait, InFileAstVariant, InFileFieldSource,
    InFileFn, InFileModuleSource, InFileSmallVecTextRange, InFileSyntaxNode, InFileSyntaxNodeRef, InFileTextRange,
};

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

#[pymethods]
impl InFileAstModule {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileModuleSource {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileAstImpl {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileAstTrait {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileAstVariant {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileFieldSource {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InFileSyntaxNodeRef {
    /// `pub file_id: HirFileId`
    #[getter]
    fn file_id(&self) -> HirFileId {
        HirFileId { inner: self.inner.file_id }
    }
}

#[pymethods]
impl InRealFileSyntaxNode {
    /// `pub file_id: EditionedFileId`
    #[getter]
    fn file_id(&self) -> EditionedFileId {
        EditionedFileId { inner: self.inner.file_id }
    }
}
