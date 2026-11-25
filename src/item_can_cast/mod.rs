use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{Item, SyntaxKind};

#[pymethods]
impl Item {
    /// `impl AstNode for Item`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::Item::can_cast(kind.inner)
    }
}
