use crate::types::{AstExpr, SemanticsImpl, TypeInfo};
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl SemanticsImpl {
    /// `fn type_of_expr(&self, expr: &ast::Expr) -> Option<TypeInfo<'db>>`
    fn type_of_expr(&self, py: Python<'_>, expr: &AstExpr) -> Option<TypeInfo> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.type_of_expr(&expr.inner))?;
        Some(TypeInfo {
            inner,
            db: self.db.clone_ref(py),
        })
    }
}
