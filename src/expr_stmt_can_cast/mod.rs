use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstExprStmt, SyntaxKind};

#[pymethods]
impl AstExprStmt {
    /// `impl AstNode for ExprStmt`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::ExprStmt::can_cast(kind.inner)
    }
}
