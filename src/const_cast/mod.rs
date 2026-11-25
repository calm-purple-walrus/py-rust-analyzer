use crate::types::{AstConst, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstConst {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<Const>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::Const::cast((*syntax.inner).clone()).map(|inner| AstConst { inner })
    }
}
