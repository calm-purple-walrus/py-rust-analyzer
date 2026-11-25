use crate::types::{AstArgList, AstMethodCallExpr};
use pyo3::prelude::*;
use ra_ap_syntax::ast::HasArgList;

#[pymethods]
impl AstMethodCallExpr {
    /// `fn arg_list(&self) -> Option<ArgList>`
    fn arg_list(&self) -> Option<AstArgList> {
        self.inner.arg_list().map(|inner| AstArgList { inner })
    }
}
