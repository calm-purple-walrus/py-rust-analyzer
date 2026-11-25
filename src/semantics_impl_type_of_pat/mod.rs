use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{AstPat, SemanticsImpl, TypeInfo};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn type_of_pat(&self, pat: &ast::Pat) -> Option<TypeInfo<'db>>`
    fn type_of_pat(&self, py: Python<'_>, pat: &AstPat) -> Option<TypeInfo> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.type_of_pat(&pat.inner))?;
        Some(TypeInfo { inner, db: self.db.clone_ref(py) })
    }
}
