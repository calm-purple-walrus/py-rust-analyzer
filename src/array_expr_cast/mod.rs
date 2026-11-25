use crate::types::{AstArrayExpr, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstArrayExpr {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<ArrayExpr>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::ArrayExpr::cast((*syntax.inner).clone())
            .map(|inner| AstArrayExpr { inner })
    }
}
