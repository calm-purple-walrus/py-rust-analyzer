use crate::types::{AstRangePat, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstRangePat {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<RangePat>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::RangePat::cast((*syntax.inner).clone())
            .map(|inner| AstRangePat { inner })
    }
}
