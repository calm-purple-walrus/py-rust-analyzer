use crate::types::{AstOrPat, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstOrPat {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<OrPat>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::OrPat::cast((*syntax.inner).clone()).map(|inner| AstOrPat { inner })
    }
}
