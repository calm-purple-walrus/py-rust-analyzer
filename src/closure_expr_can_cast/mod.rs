use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstClosureExpr, SyntaxKind};

#[pymethods]
impl AstClosureExpr {
    /// `impl AstNode for ClosureExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::ClosureExpr::can_cast(kind.inner)
    }
}
