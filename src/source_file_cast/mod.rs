use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{SourceFile, SyntaxNode};

#[pymethods]
impl SourceFile {
    /// `impl AstNode for SourceFile`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<SourceFile> {
        ra_ap_syntax::ast::SourceFile::cast((*syntax.inner).clone()).map(|inner| SourceFile { inner })
    }
}
