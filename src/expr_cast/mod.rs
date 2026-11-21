use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstExpr, SyntaxNode};

#[pymethods]
impl AstExpr {
    /// `impl AstNode for Expr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstExpr> {
        ra_ap_syntax::ast::Expr::cast((*syntax.inner).clone()).map(|inner| AstExpr { inner })
    }
}
