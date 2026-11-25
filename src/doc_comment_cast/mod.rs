use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstDocComment, SyntaxNode};

#[pymethods]
impl AstDocComment {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<DocComment>`. It takes a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::DocComment::cast((*syntax.inner).clone()).map(|inner| AstDocComment { inner })
    }
}
