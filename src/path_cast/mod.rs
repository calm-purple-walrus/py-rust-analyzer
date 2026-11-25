use crate::types::{AstPath, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstPath {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<Path>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::Path::cast((*syntax.inner).clone()).map(|inner| AstPath { inner })
    }
}
