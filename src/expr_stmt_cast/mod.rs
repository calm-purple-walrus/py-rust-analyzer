use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstExprStmt, SyntaxNode};

#[pymethods]
impl AstExprStmt {
    /// `impl AstNode for ExprStmt`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstExprStmt> {
        ra_ap_syntax::ast::ExprStmt::cast((*syntax.inner).clone()).map(|inner| AstExprStmt { inner })
    }
}
