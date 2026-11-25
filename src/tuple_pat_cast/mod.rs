use crate::types::{AstTuplePat, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstTuplePat {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<TuplePat>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::TuplePat::cast((*syntax.inner).clone())
            .map(|inner| AstTuplePat { inner })
    }
}
