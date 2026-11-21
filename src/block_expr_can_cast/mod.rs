use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstBlockExpr, SyntaxKind};

#[pymethods]
impl AstBlockExpr {
    /// `impl AstNode for BlockExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::BlockExpr::can_cast(kind.inner)
    }
}
