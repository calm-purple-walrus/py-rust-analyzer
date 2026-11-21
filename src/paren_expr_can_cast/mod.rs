use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstParenExpr, SyntaxKind};

#[pymethods]
impl AstParenExpr {
    /// `impl AstNode for ParenExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::ParenExpr::can_cast(kind.inner)
    }
}
