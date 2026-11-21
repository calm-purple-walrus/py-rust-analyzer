use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstCallExpr, SyntaxNode};

#[pymethods]
impl AstCallExpr {
    /// `impl AstNode for CallExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstCallExpr> {
        ra_ap_syntax::ast::CallExpr::cast((*syntax.inner).clone()).map(|inner| AstCallExpr { inner })
    }
}
