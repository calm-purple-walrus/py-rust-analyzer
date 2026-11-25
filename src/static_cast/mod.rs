use crate::types::{AstStatic, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstStatic {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<Static>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::Static::cast((*syntax.inner).clone()).map(|inner| AstStatic { inner })
    }
}
