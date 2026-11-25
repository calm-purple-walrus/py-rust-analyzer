use crate::types::{AstReturnExpr, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstReturnExpr {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<ReturnExpr>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::ReturnExpr::cast((*syntax.inner).clone())
            .map(|inner| AstReturnExpr { inner })
    }
}
