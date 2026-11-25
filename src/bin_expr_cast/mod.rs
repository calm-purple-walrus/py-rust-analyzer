use crate::types::{AstBinExpr, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstBinExpr {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<BinExpr>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::BinExpr::cast((*syntax.inner).clone()).map(|inner| AstBinExpr { inner })
    }
}
