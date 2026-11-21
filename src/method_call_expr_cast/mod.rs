use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstMethodCallExpr, SyntaxNode};

#[pymethods]
impl AstMethodCallExpr {
    /// `impl AstNode for MethodCallExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstMethodCallExpr> {
        ra_ap_syntax::ast::MethodCallExpr::cast((*syntax.inner).clone()).map(|inner| AstMethodCallExpr { inner })
    }
}
