use crate::types::{AstArgList, AstCallExpr};
use pyo3::prelude::*;
use ra_ap_syntax::ast::HasArgList;

#[pymethods]
impl AstCallExpr {
    /// `fn arg_list(&self) -> Option<ArgList>`
    fn arg_list(&self) -> Option<AstArgList> {
        self.inner.arg_list().map(|inner| AstArgList { inner })
    }
}
