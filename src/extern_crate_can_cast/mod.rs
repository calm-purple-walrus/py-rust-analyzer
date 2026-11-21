use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstExternCrate, SyntaxKind};

#[pymethods]
impl AstExternCrate {
    /// `impl AstNode for ExternCrate`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::ExternCrate::can_cast(kind.inner)
    }
}
