use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{AstMethodCallExpr, Function, SemanticsImpl};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn resolve_method_call(&self, call: &ast::MethodCallExpr) -> Option<Function>`
    fn resolve_method_call(&self, py: Python<'_>, call: &AstMethodCallExpr) -> Option<Function> {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.resolve_method_call(&call.inner)).map(|inner| Function { inner })
    }
}
