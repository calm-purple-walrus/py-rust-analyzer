use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstLetStmt, SyntaxNode};

#[pymethods]
impl AstLetStmt {
    /// `impl AstNode for LetStmt`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstLetStmt> {
        ra_ap_syntax::ast::LetStmt::cast((*syntax.inner).clone()).map(|inner| AstLetStmt { inner })
    }
}
