use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstAttr, SyntaxNode};

#[pymethods]
impl AstAttr {
    /// `impl AstNode for Attr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstAttr> {
        ra_ap_syntax::ast::Attr::cast((*syntax.inner).clone()).map(|inner| AstAttr { inner })
    }
}
