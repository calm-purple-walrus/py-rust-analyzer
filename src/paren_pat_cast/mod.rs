use crate::types::{AstParenPat, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstParenPat {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<ParenPat>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::ParenPat::cast((*syntax.inner).clone())
            .map(|inner| AstParenPat { inner })
    }
}
