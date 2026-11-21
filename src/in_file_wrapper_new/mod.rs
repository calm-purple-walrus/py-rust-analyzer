use pyo3::prelude::*;

use crate::types::{
    HirFileId, InFileSyntaxNode, InFileTextRange, InMacroFileSyntaxToken, MacroCallId, SyntaxNode, SyntaxToken, TextRange,
};

#[pymethods]
impl InFileTextRange {
    /// `pub fn new(file_id: FileKind, value: T) -> InFileWrapper<FileKind, T>`, with `T` the `TextRange`.
    #[staticmethod]
    fn new(file_id: &HirFileId, value: &TextRange) -> InFileTextRange {
        InFileTextRange { inner: ra_ap_hir::InFile::new(file_id.inner, value.inner) }
    }
}

#[pymethods]
impl InMacroFileSyntaxToken {
    /// `pub fn new(file_id: FileKind, value: T) -> InFileWrapper<FileKind, T>`, with `FileKind` the
    /// `MacroCallId` and `T` the `SyntaxToken`. It takes the token, so it is given a copy.
    #[staticmethod]
    fn new(file_id: &MacroCallId, value: &SyntaxToken) -> InMacroFileSyntaxToken {
        InMacroFileSyntaxToken { inner: ra_ap_hir_expand::InMacroFile::new(file_id.inner, (*value.inner).clone()) }
    }
}

#[pymethods]
impl InFileSyntaxNode {
    /// `pub fn new(file_id: FileKind, value: T) -> InFileWrapper<FileKind, T>`, with `FileKind` the `HirFileId` and
    /// `T` the `SyntaxNode`. It takes the node, so it is given a copy.
    #[staticmethod]
    fn new(file_id: &HirFileId, value: &SyntaxNode) -> InFileSyntaxNode {
        InFileSyntaxNode { inner: ra_ap_hir::InFile::new(file_id.inner, (*value.inner).clone()) }
    }
}
