use crate::types::{AstType, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstType {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<Type>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::Type::cast((*syntax.inner).clone()).map(|inner| AstType { inner })
    }
}
