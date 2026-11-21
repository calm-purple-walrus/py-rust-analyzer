use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstFieldExpr, SyntaxNode};

#[pymethods]
impl AstFieldExpr {
    /// `impl AstNode for FieldExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstFieldExpr> {
        ra_ap_syntax::ast::FieldExpr::cast((*syntax.inner).clone()).map(|inner| AstFieldExpr { inner })
    }
}
