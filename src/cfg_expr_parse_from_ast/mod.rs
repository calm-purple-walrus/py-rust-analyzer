use pyo3::prelude::*;

use crate::types::{AstCfgPredicate, CfgExpr};

#[pymethods]
impl CfgExpr {
    /// `pub fn parse_from_ast(ast: syntax::ast::CfgPredicate) -> CfgExpr`. It takes the predicate, so it is given
    /// a copy.
    #[staticmethod]
    fn parse_from_ast(ast: &AstCfgPredicate) -> CfgExpr {
        CfgExpr { inner: ra_ap_hir::CfgExpr::parse_from_ast(ast.inner.clone()) }
    }
}
