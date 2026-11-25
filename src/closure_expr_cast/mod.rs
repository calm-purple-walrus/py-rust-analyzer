use crate::held::Held;
use crate::types::{AstClosureExpr, SyntaxNode};
use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

#[pymethods]
impl AstClosureExpr {
    /// `AstNode::cast(syntax: SyntaxNode) -> Option<ClosureExpr>`. It takes a copy of the node.
    #[staticmethod]
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        ra_ap_syntax::ast::ClosureExpr::cast((*syntax.inner).clone()).map(|inner| AstClosureExpr {
            inner: Held::Own(inner),
        })
    }
}
