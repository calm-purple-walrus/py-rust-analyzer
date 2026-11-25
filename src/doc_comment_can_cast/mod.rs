use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstDocComment, SyntaxKind};

#[pymethods]
impl AstDocComment {
    /// `impl AstNode for DocComment`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::DocComment::can_cast(kind.inner)
    }
}
