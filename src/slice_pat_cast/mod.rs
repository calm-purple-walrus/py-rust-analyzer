use crate::types::{AstSlicePat, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstSlicePat {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<SlicePat>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::SlicePat::cast((*syntax.inner).clone())
            .map(|inner| AstSlicePat { inner })
    }
}
