use crate::types::{AstTupleStructPat, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstTupleStructPat {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<TupleStructPat>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::TupleStructPat::cast((*syntax.inner).clone())
            .map(|inner| AstTupleStructPat { inner })
    }
}
