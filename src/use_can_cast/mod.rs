use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstUse, SyntaxKind};

#[pymethods]
impl AstUse {
    /// `impl AstNode for Use`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::Use::can_cast(kind.inner)
    }
}
