use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstItemList, SyntaxKind};

#[pymethods]
impl AstItemList {
    /// `impl AstNode for ItemList`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::ItemList::can_cast(kind.inner)
    }
}
