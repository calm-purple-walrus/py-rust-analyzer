use pyo3::prelude::*;
use ra_ap_hir::attach_db;

use crate::types::{AstPath, PathResolution, SemanticsImpl};

#[pymethods]
impl SemanticsImpl {
    /// `pub fn resolve_path(&self, path: &ast::Path) -> Option<PathResolution<'db>>`
    fn resolve_path(&self, py: Python<'_>, path: &AstPath) -> Option<PathResolution> {
        let db = self.db.borrow(py);
        let inner = attach_db(&db.db, || self.inner.resolve_path(&path.inner))?;
        Some(PathResolution { inner, _db: self.db.clone_ref(py) })
    }
}
