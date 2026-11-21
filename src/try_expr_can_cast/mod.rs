use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstTryExpr, SyntaxKind};

#[pymethods]
impl AstTryExpr {
    /// `impl AstNode for TryExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::TryExpr::can_cast(kind.inner)
    }
}
