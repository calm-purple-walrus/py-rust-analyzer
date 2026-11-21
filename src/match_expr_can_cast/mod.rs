use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstMatchExpr, SyntaxKind};

#[pymethods]
impl AstMatchExpr {
    /// `impl AstNode for MatchExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::MatchExpr::can_cast(kind.inner)
    }
}
