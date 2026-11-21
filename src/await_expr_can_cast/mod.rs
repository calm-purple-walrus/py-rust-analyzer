use pyo3::prelude::*;
use ra_ap_syntax::AstNode;

use crate::types::{AstAwaitExpr, SyntaxKind};

#[pymethods]
impl AstAwaitExpr {
    /// `impl AstNode for AwaitExpr`: `fn can_cast(kind: SyntaxKind) -> bool`
    #[staticmethod]
    fn can_cast(kind: &SyntaxKind) -> bool {
        ra_ap_syntax::ast::AwaitExpr::can_cast(kind.inner)
    }
}
