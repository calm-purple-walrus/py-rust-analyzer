use crate::types::{AstIdentPat, ModuleDef, SemanticsImpl};
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl SemanticsImpl {
    /// `fn resolve_bind_pat_to_const(&self, pat: &ast::IdentPat) -> Option<ModuleDef>`
    fn resolve_bind_pat_to_const(&self, py: Python<'_>, pat: &AstIdentPat) -> Option<ModuleDef> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.resolve_bind_pat_to_const(&pat.inner))?;
        Some(ModuleDef { inner })
    }
}
