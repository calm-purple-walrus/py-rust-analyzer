use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{SourceFile, SyntaxKind};

#[pymethods]
impl SourceFile {
    /// `impl AstNode for SourceFile`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::SourceFile::can_cast(kind.inner)
    }
}
