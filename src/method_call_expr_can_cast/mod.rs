use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstMethodCallExpr, SyntaxKind};

#[pymethods]
impl AstMethodCallExpr {
    /// `impl AstNode for MethodCallExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::MethodCallExpr::can_cast(kind.inner)
    }
}
