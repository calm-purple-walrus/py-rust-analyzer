use crate::types::{AstTypeAlias, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstTypeAlias {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<TypeAlias>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::TypeAlias::cast((*syntax.inner).clone())
            .map(|inner| AstTypeAlias { inner })
    }
}
