use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstLetExpr, SyntaxNode};

#[pymethods]
impl AstLetExpr {
    /// `impl AstNode for LetExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is
    /// given a copy.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<AstLetExpr> {
        ra_ap_syntax::ast::LetExpr::cast((*syntax.inner).clone()).map(|inner| AstLetExpr { inner })
    }
}
