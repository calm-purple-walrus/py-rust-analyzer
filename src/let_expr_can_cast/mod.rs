use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstLetExpr, SyntaxKind};

#[pymethods]
impl AstLetExpr {
    /// `impl AstNode for LetExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::LetExpr::can_cast(kind.inner)
    }
}
