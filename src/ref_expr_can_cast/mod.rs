use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstRefExpr, SyntaxKind};

#[pymethods]
impl AstRefExpr {
    /// `impl AstNode for RefExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::RefExpr::can_cast(kind.inner)
    }
}
