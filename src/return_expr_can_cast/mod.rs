use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstReturnExpr, SyntaxKind};

#[pymethods]
impl AstReturnExpr {
    /// `impl AstNode for ReturnExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::ReturnExpr::can_cast(kind.inner)
    }
}
