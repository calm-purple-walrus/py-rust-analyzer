use crate::types::Type;
use pyo3::prelude::*;
use ra_ap_hir::attach_db;

#[pymethods]
impl Type {
    /// `fn is_unknown(&self) -> bool`
    fn is_unknown(&self, py: Python<'_>) -> bool {
        let db = self.db.borrow(py);
        attach_db(&db.db, || self.inner.is_unknown())
    }
}
