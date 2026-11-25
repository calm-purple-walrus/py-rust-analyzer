use crate::types::{AstRefPat, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstRefPat {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<RefPat>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::RefPat::cast((*syntax.inner).clone()).map(|inner| AstRefPat { inner })
    }
}
