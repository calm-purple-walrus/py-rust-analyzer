use crate::types::{AstExternBlock, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstExternBlock {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<ExternBlock>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::ExternBlock::cast((*syntax.inner).clone())
            .map(|inner| AstExternBlock { inner })
    }
}
