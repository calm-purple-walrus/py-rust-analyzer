use crate::types::{AstMatchArm, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstMatchArm {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<MatchArm>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::MatchArm::cast((*syntax.inner).clone())
            .map(|inner| AstMatchArm { inner })
    }
}
