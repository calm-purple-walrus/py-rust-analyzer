use crate::types::{AstRangeExpr, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstRangeExpr {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<RangeExpr>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::RangeExpr::cast((*syntax.inner).clone())
            .map(|inner| AstRangeExpr { inner })
    }
}
